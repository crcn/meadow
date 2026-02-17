use std::sync::Arc;

use ai_client::OpenAi;
use neo4rs::Graph;
use uuid::Uuid;

use super::agent::{investigate, ProposedResource};
use crate::domains::graph::{models::Resource, queries};
use crate::error::{Error, Result};

const LOAD_MORE_PROMPT: &str = r#"You are finding additional learning resources for a specific topic in a knowledge graph.

The learner wants MORE resources beyond what they already have. Search for NEW, different resources on this topic. Do NOT repeat any of the existing resources listed below.

Return your results as a JSON array with a single entry:
[
  {
    "title": "<topic title>",
    "description": "<topic description>",
    "movement": "FOUNDATION",
    "resources": [
      {
        "type": "youtube|article|guide|tutorial",
        "youtube_id": "optional",
        "url": "required",
        "title": "Resource title",
        "channel": "optional",
        "reason": "Why this resource is good for learning this concept"
      }
    ]
  }
]

Find 2-3 new resources. Prefer different formats than what's already there (if they had YouTube, try articles, and vice versa). Make sure resources actually exist."#;

/// Load more resources for a node, appending new AI-found ones to the existing list.
pub async fn load_more_resources(
    ai_agent: &OpenAi,
    memgraph: Arc<Graph>,
    tavily_api_key: &str,
    youtube_api_key: &str,
    node_id: Uuid,
    node_title: &str,
    node_description: &str,
    existing_resources: &[Resource],
    max_turns: usize,
) -> Result<Vec<Resource>> {
    tracing::info!(
        node_id = %node_id,
        node_title = %node_title,
        existing_count = existing_resources.len(),
        "Loading more resources"
    );

    let existing_list = existing_resources
        .iter()
        .map(|r| format!("- {} ({}): {}", r.title, r.resource_type, r.url.as_deref().unwrap_or("no url")))
        .collect::<Vec<_>>()
        .join("\n");

    let user_prompt = format!(
        "Find more resources for the topic: \"{}\" — {}\n\nExisting resources (DO NOT repeat these):\n{}\n\nFind new, different resources to add.",
        node_title, node_description, existing_list
    );

    let proposals = investigate(
        ai_agent,
        memgraph.clone(),
        tavily_api_key,
        youtube_api_key,
        LOAD_MORE_PROMPT,
        &user_prompt,
        max_turns,
        false,
    )
    .await?;

    let new_resources: Vec<Resource> = proposals
        .into_iter()
        .flat_map(|p| p.resources)
        .map(|r: ProposedResource| r.into())
        .collect();

    if new_resources.is_empty() {
        return Err(Error::Ai("AI did not find any new resources".into()));
    }

    // Append new resources to existing ones
    let mut all_resources = existing_resources.to_vec();
    all_resources.extend(new_resources);

    // Update the node in Memgraph
    queries::update_node_resources(&memgraph, node_id, &all_resources).await?;

    tracing::info!(
        node_id = %node_id,
        total_count = all_resources.len(),
        "Resources loaded"
    );

    Ok(all_resources)
}
