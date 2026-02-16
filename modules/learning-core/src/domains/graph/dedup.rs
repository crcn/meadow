use ai_client::traits::EmbedAgent;
use neo4rs::Graph;
use uuid::Uuid;

use super::models::Node;
use super::queries;
use crate::error::Result;

const DEDUP_THRESHOLD: f64 = 0.85;

/// Check if a proposed node already exists in the topic graph.
///
/// Returns Some(existing_node) if a match is found, None if the node is new.
pub async fn find_duplicate(
    graph: &Graph,
    embed_agent: &dyn EmbedAgent,
    title: &str,
    description: &str,
    topic_root_id: Uuid,
) -> Result<Option<Node>> {
    let text = format!("{} {}", title, description);
    let embedding = embed_agent.embed(text.to_string()).await
        .map_err(|e| crate::error::Error::Ai(e.to_string()))?;

    let matches = queries::vector_search_nodes(graph, &embedding, topic_root_id, 3).await?;

    if let Some((node, similarity)) = matches.first() {
        if *similarity >= DEDUP_THRESHOLD {
            tracing::info!(
                "Dedup: '{}' matches existing node '{}' (similarity: {:.3})",
                title, node.title, similarity
            );
            return Ok(Some(node.clone()));
        }
    }

    Ok(None)
}
