use ai_client::traits::EmbedAgent;
use neo4rs::Graph;
use uuid::Uuid;

use super::models::TopicRoot;
use super::queries;
use crate::error::Result;

const MATCH_THRESHOLD: f64 = 0.85;

/// Match an interest string to an existing TopicRoot, or create a new one.
///
/// 1. Embed the interest text
/// 2. Vector search against existing TopicRoots
/// 3. If similarity > 0.85 → use existing
/// 4. Otherwise → create new TopicRoot
pub async fn match_or_create_topic(
    graph: &Graph,
    embed_agent: &dyn EmbedAgent,
    interest: &str,
) -> Result<(TopicRoot, bool)> {
    let embedding = embed_agent.embed(interest.to_string()).await
        .map_err(|e| crate::error::Error::Ai(e.to_string()))?;

    // Search existing topic roots
    let matches = queries::vector_search_topic_roots(graph, &embedding, 3).await?;

    if let Some((topic_root, similarity)) = matches.first() {
        if *similarity >= MATCH_THRESHOLD {
            tracing::info!(
                "Matched interest '{}' to existing topic '{}' (similarity: {:.3})",
                interest, topic_root.name, similarity
            );
            return Ok((topic_root.clone(), false));
        }
    }

    // Create new topic root
    let id = Uuid::new_v4();
    let topic_root = queries::create_topic_root(
        graph,
        id,
        interest,
        &format!("Topic graph for: {}", interest),
        &embedding,
    )
    .await?;

    tracing::info!("Created new topic root '{}' (id: {})", interest, id);
    Ok((topic_root, true))
}
