use ai_client::traits::EmbedAgent;
use ai_client::OpenAi;
use neo4rs::Graph;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::agent;
use super::prompts::{INVESTIGATION_SYSTEM_PROMPT, STARTING_POINTS_PROMPT};
use crate::domains::graph::models::{Movement, Resource, TopicGraph};
use crate::domains::graph::{assembler, queries};
use crate::error::Result;

/// Generate starting points for a new topic.
/// Creates nodes connected to the TopicRoot via STARTS_WITH edges.
pub async fn generate_starting_points(
    ai_agent: &OpenAi,
    embed_agent: &dyn EmbedAgent,
    memgraph: Arc<Graph>,
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
    topic_name: &str,
    tavily_api_key: &str,
    youtube_api_key: &str,
    max_turns: usize,
) -> Result<TopicGraph> {
    tracing::info!(
        topic = %topic_name,
        topic_root_id = %topic_root_id,
        member_id = %member_id,
        "Generating starting points for new topic"
    );

    let prompt = format!(
        "{}\n\nThe topic is: \"{}\"\n\nGenerate 3-5 starting points. Each should be a fundamentally different entry angle into this subject.",
        STARTING_POINTS_PROMPT, topic_name
    );

    let proposals = agent::investigate(
        ai_agent,
        memgraph.clone(),
        tavily_api_key,
        youtube_api_key,
        INVESTIGATION_SYSTEM_PROMPT,
        &prompt,
        max_turns,
        false,
    )
    .await?;

    tracing::info!(
        count = proposals.len(),
        "Persisting starting point nodes"
    );

    // Persist each starting point
    for (i, proposal) in proposals.iter().enumerate() {
        let movement = Movement::from_relationship_type(&proposal.movement)
            .unwrap_or(Movement::Foundation);
        let resources: Vec<Resource> = proposal.resources.iter().cloned().map(Into::into).collect();

        tracing::info!(
            index = i,
            title = %proposal.title,
            movement = %proposal.movement,
            resources = resources.len(),
            "Embedding and saving starting point"
        );

        let embedding = embed_agent
            .embed(format!("{} {}", proposal.title, proposal.description))
            .await
            .map_err(|e| crate::error::Error::Ai(e.to_string()))?;

        let node_id = Uuid::new_v4();
        queries::create_node(
            &memgraph,
            node_id,
            &proposal.title,
            &proposal.description,
            topic_root_id,
            &resources,
            &embedding,
            1, // starting points are depth 1
        )
        .await?;

        // Connect to topic root via the AI-assigned movement type
        queries::create_edge(&memgraph, topic_root_id, node_id, movement).await?;

        tracing::info!(
            node_id = %node_id,
            title = %proposal.title,
            movement = %movement.as_relationship_type(),
            "Starting point node created"
        );
    }

    // Set learner position to topic root
    sqlx::query(
        "INSERT INTO learner_position (member_id, topic_root_id, current_node_id) VALUES ($1, $2, $3) ON CONFLICT (member_id, topic_root_id) DO UPDATE SET current_node_id = $3, updated_at = now()"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .bind(topic_root_id)
    .execute(db)
    .await?;

    assembler::assemble_topic_graph(&memgraph, db, member_id, topic_root_id).await
}
