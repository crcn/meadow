use ai_client::traits::EmbedAgent;
use ai_client::OpenAi;
use neo4rs::Graph;
use schemars::JsonSchema;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::agent;
use super::prompts::{EXPANSION_SUGGESTIONS_PROMPT, INVESTIGATION_SYSTEM_PROMPT, STEERED_INVESTIGATION_PROMPT};
use crate::domains::graph::models::{Movement, Resource, TopicGraph};
use crate::domains::graph::{assembler, dedup, queries};
use crate::error::Result;

/// Generate proposals for a node. Checks existing edges first,
/// runs AI investigation if the graph is sparse.
pub async fn generate_proposals(
    ai_agent: &OpenAi,
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
    steering_prompt: Option<&str>,
) -> Result<TopicGraph> {
    let steered = steering_prompt.is_some();

    // Check existing outgoing edges (skip check when steered — user explicitly asked)
    let existing_count = if steered {
        0 // force AI investigation
    } else {
        count_outgoing_edges(&memgraph, from_node_id).await?
    };

    tracing::info!(
        from_node = %from_node_title,
        from_node_id = %from_node_id,
        existing_edges = existing_count,
        steered = steered,
        "Generating proposals (need AI: {})",
        existing_count < 3
    );

    if existing_count < 3 {
        // Build traversal path context for the AI
        let path_context = build_path_context(
            &memgraph, db, member_id, topic_root_id, from_node_id,
        ).await?;

        // Get the current node's depth
        let from_node_depth = queries::get_node(&memgraph, from_node_id)
            .await?
            .map(|n| n.depth)
            .unwrap_or(1);

        let (system_prompt, prompt) = if let Some(direction) = steering_prompt {
            (
                STEERED_INVESTIGATION_PROMPT,
                format!(
                    "{}\n\nCurrent node: \"{}\" at depth {}.\n\
                     The learner wants to explore: \"{}\"\n\
                     Generate 3 proposals aligned with this direction.",
                    path_context, from_node_title, from_node_depth, direction
                ),
            )
        } else {
            (
                INVESTIGATION_SYSTEM_PROMPT,
                format!(
                    "{}\n\nCurrent node: \"{}\" at depth {}.\n\
                     Generate 3 proposals. The DEEPER proposal must be noticeably more advanced than depth {}.",
                    path_context, from_node_title, from_node_depth, from_node_depth
                ),
            )
        };

        let proposals = agent::investigate(
            ai_agent,
            memgraph.clone(),
            tavily_api_key,
            youtube_api_key,
            system_prompt,
            &prompt,
            max_turns,
            steered,
        )
        .await?;

        // Persist each proposal (dedup first)
        for proposal in &proposals {
            let movement = Movement::from_relationship_type(&proposal.movement)
                .unwrap_or(Movement::Foundation);
            let resources: Vec<Resource> = proposal.resources.iter().cloned().map(Into::into).collect();

            // Check for duplicate
            tracing::info!(title = %proposal.title, movement = %proposal.movement, "Checking for duplicate node");
            let existing = dedup::find_duplicate(
                &memgraph,
                embed_agent,
                &proposal.title,
                &proposal.description,
                topic_root_id,
            )
            .await?;

            // Compute depth for the new node
            let proposal_depth = match movement {
                Movement::Deeper => from_node_depth + 1,
                Movement::Broader | Movement::Practice | Movement::Inspire => from_node_depth,
                Movement::Foundation => (from_node_depth - 1).max(1),
            };

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
                    proposal_depth,
                )
                .await?;

                tracing::info!(
                    node_id = %new_id,
                    title = %proposal.title,
                    movement = %proposal.movement,
                    depth = proposal_depth,
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

/// Structured response for expansion suggestions.
#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
struct SuggestionsResponse {
    suggestions: Vec<String>,
}

/// Generate 3 contextual expansion direction suggestions for a node.
/// Lightweight single-call extraction — no tool-calling loop.
pub async fn suggest_expansions(
    ai_agent: &OpenAi,
    memgraph: Arc<Graph>,
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
    node_id: Uuid,
    node_title: &str,
) -> Result<Vec<String>> {
    let path_context = build_path_context(&memgraph, db, member_id, topic_root_id, node_id).await?;

    let node_depth = queries::get_node(&memgraph, node_id)
        .await?
        .map(|n| n.depth)
        .unwrap_or(1);

    let node_description = queries::get_node(&memgraph, node_id)
        .await?
        .map(|n| n.description)
        .unwrap_or_default();

    let user_prompt = format!(
        "{}\n\nCurrent node: \"{}\" — {}\nDepth: {}\n\nGenerate 3 expansion direction suggestions.",
        path_context, node_title, node_description, node_depth
    );

    tracing::info!(
        node = %node_title,
        depth = node_depth,
        "Generating expansion suggestions"
    );

    let response: SuggestionsResponse = ai_agent
        .extract(ai_agent.model(), EXPANSION_SUGGESTIONS_PROMPT, &user_prompt)
        .await
        .map_err(|e| crate::error::Error::Ai(e.to_string()))?;

    tracing::info!(
        suggestions = response.suggestions.len(),
        "Expansion suggestions generated"
    );

    Ok(response.suggestions)
}

/// Build a textual path summary from the learner's traversal history.
pub(crate) async fn build_path_context(
    memgraph: &Graph,
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
    _from_node_id: Uuid,
) -> Result<String> {
    // Get topic root name
    let topic_root = queries::find_topic_root_by_id(memgraph, topic_root_id).await?;
    let root_name = topic_root.map(|t| t.name).unwrap_or_else(|| "Topic".to_string());

    // Get traversal history
    let steps = queries::fetch_traversal_path(db, member_id, topic_root_id).await?;

    if steps.is_empty() {
        return Ok(format!(
            "The learner's path (earliest → most recent):\n1. \"{}\" (ROOT)",
            root_name
        ));
    }

    let mut path_lines = vec![format!("1. \"{}\" (ROOT)", root_name)];

    for (i, step) in steps.iter().enumerate() {
        // Look up node title
        let title = if let Some(node) = queries::get_node(memgraph, step.node_id).await? {
            node.title
        } else {
            step.node_id.to_string()
        };

        let movement_str = step.movement.as_deref().unwrap_or("TRAVERSED");
        let depth = i + 1;
        path_lines.push(format!(
            "{}. \"{}\" ({}, depth {})",
            i + 2, title, movement_str, depth
        ));
    }

    Ok(format!(
        "The learner's path (earliest → most recent):\n{}",
        path_lines.join("\n")
    ))
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
