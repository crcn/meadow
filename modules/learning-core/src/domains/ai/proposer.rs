use ai_client::traits::{Agent, EmbedAgent};
use neo4rs::Graph;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::agent;
use super::prompts::INVESTIGATION_SYSTEM_PROMPT;
use crate::domains::graph::models::{Movement, Resource, TopicGraph};
use crate::domains::graph::{assembler, dedup, queries};
use crate::error::Result;

/// Generate proposals for a node. Checks existing edges first,
/// runs AI investigation if the graph is sparse.
pub async fn generate_proposals<A: Agent>(
    ai_agent: &A,
    embed_agent: &dyn EmbedAgent,
    memgraph: Arc<Graph>,
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
    from_node_id: Uuid,
    from_node_title: &str,
    tavily_api_key: &str,
    youtube_api_key: &str,
    max_turns: usize,
) -> Result<TopicGraph> {
    // Check existing outgoing edges
    let existing_count = count_outgoing_edges(&memgraph, from_node_id).await?;

    tracing::info!(
        from_node = %from_node_title,
        from_node_id = %from_node_id,
        existing_edges = existing_count,
        "Generating proposals (need AI: {})",
        existing_count < 3
    );

    if existing_count < 3 {
        // Graph is sparse — run AI investigation
        let prompt = format!(
            "The learner is currently on the node: \"{}\"\n\nGenerate 3 proposals for where they could go next. Investigate real learning resources.",
            from_node_title
        );

        let proposals = agent::investigate(
            ai_agent,
            memgraph.clone(),
            tavily_api_key,
            youtube_api_key,
            INVESTIGATION_SYSTEM_PROMPT,
            &prompt,
            max_turns,
        )
        .await?;

        // Persist each proposal (dedup first)
        for proposal in &proposals {
            let movement = Movement::from_relationship_type(&proposal.movement)
                .unwrap_or(Movement::Supports);
            let resources: Vec<Resource> = proposal.resources.iter().cloned().map(Into::into).collect();

            // Check for duplicate
            tracing::info!(title = %proposal.title, "Checking for duplicate node");
            let existing = dedup::find_duplicate(
                &memgraph,
                embed_agent,
                &proposal.title,
                &proposal.description,
                topic_root_id,
            )
            .await?;

            let node_id = if let Some(existing_node) = existing {
                tracing::info!(
                    title = %proposal.title,
                    existing_id = %existing_node.id,
                    "Dedup: reusing existing node"
                );
                existing_node.id
            } else {
                // Create new node
                let embedding = embed_agent
                    .embed(format!("{} {}", proposal.title, proposal.description))
                    .await
                    .map_err(|e| crate::error::Error::Ai(e.to_string()))?;

                let new_id = Uuid::new_v4();
                queries::create_node(
                    &memgraph,
                    new_id,
                    &proposal.title,
                    &proposal.description,
                    topic_root_id,
                    &resources,
                    &embedding,
                )
                .await?;

                tracing::info!(
                    node_id = %new_id,
                    title = %proposal.title,
                    movement = %proposal.movement,
                    resources = resources.len(),
                    "Created new proposal node"
                );
                new_id
            };

            // Create edge
            queries::create_edge(&memgraph, from_node_id, node_id, movement).await?;
        }
    } else {
        tracing::info!("Sufficient edges exist, skipping AI investigation");
    }

    // Return the assembled graph
    assembler::assemble_topic_graph(&memgraph, db, member_id, topic_root_id).await
}

async fn count_outgoing_edges(graph: &Graph, node_id: Uuid) -> Result<i64> {
    let mut result = graph
        .execute(
            neo4rs::query("MATCH ({id: $id})-->() RETURN count(*) AS cnt")
                .param("id", node_id.to_string()),
        )
        .await?;

    if let Some(row) = result.next().await? {
        Ok(row.get("cnt")?)
    } else {
        Ok(0)
    }
}
