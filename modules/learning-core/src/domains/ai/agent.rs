use ai_client::traits::{Agent, PromptBuilder};
use neo4rs::Graph;
use std::sync::Arc;

use super::tools::{
    existing_nodes::ExistingNodesTool, tavily::TavilySearchTool,
    youtube_details::YoutubeDetailsTool, youtube_search::YoutubeSearchTool,
};
use crate::domains::graph::models::Resource;
use crate::error::{Error, Result};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProposedNode {
    pub title: String,
    pub description: String,
    pub movement: String,
    pub resources: Vec<ProposedResource>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProposedResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub channel: Option<String>,
    pub reason: String,
}

impl From<ProposedResource> for Resource {
    fn from(r: ProposedResource) -> Self {
        Resource {
            resource_type: r.resource_type,
            youtube_id: r.youtube_id,
            url: r.url,
            title: r.title,
            channel: r.channel,
            reason: r.reason,
        }
    }
}

/// Run the AI investigation agent with all tools configured.
/// Returns a list of proposed nodes with resources.
pub async fn investigate<A: Agent>(
    agent: &A,
    memgraph: Arc<Graph>,
    tavily_api_key: &str,
    youtube_api_key: &str,
    system_prompt: &str,
    user_prompt: &str,
    max_turns: usize,
) -> Result<Vec<ProposedNode>> {
    tracing::info!(
        max_turns = max_turns,
        prompt_len = user_prompt.len(),
        "Starting AI investigation"
    );

    let agent = agent
        .clone()
        .tool(TavilySearchTool::new(tavily_api_key.to_string()))
        .tool(YoutubeSearchTool::new(youtube_api_key.to_string()))
        .tool(YoutubeDetailsTool::new(youtube_api_key.to_string()))
        .tool(ExistingNodesTool::new(memgraph));

    let start = std::time::Instant::now();

    let response = agent
        .prompt(user_prompt)
        .preamble(system_prompt)
        .multi_turn(max_turns)
        .send()
        .await
        .map_err(|e| Error::Ai(e.to_string()))?;

    tracing::info!(
        response_len = response.len(),
        elapsed_ms = start.elapsed().as_millis() as u64,
        "AI investigation complete"
    );

    // Parse JSON array from the response
    let proposals = parse_proposals(&response)?;

    for (i, p) in proposals.iter().enumerate() {
        tracing::info!(
            index = i,
            title = %p.title,
            movement = %p.movement,
            resources = p.resources.len(),
            "Parsed proposal"
        );
    }

    Ok(proposals)
}

fn parse_proposals(response: &str) -> Result<Vec<ProposedNode>> {
    // Find JSON array in the response (it might be wrapped in markdown code blocks)
    let json_str = extract_json_array(response)
        .ok_or_else(|| Error::Ai("No JSON array found in agent response".into()))?;

    serde_json::from_str(&json_str)
        .map_err(|e| Error::Ai(format!("Failed to parse proposals: {}", e)))
}

fn extract_json_array(text: &str) -> Option<String> {
    // Try to find a JSON array, possibly within code blocks
    let text = text.trim();

    // If the whole thing is a JSON array
    if text.starts_with('[') {
        return Some(text.to_string());
    }

    // Look for ```json ... ``` blocks
    if let Some(start) = text.find("```json") {
        let after_marker = &text[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return Some(after_marker[..end].trim().to_string());
        }
    }

    // Look for ``` ... ``` blocks
    if let Some(start) = text.find("```") {
        let after_marker = &text[start + 3..];
        if let Some(end) = after_marker.find("```") {
            let block = after_marker[..end].trim();
            if block.starts_with('[') {
                return Some(block.to_string());
            }
        }
    }

    // Last resort: find first [ and last ]
    let start = text.find('[')?;
    let end = text.rfind(']')?;
    if end > start {
        Some(text[start..=end].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_array_plain() {
        let input = r#"[{"title": "test"}]"#;
        assert_eq!(extract_json_array(input), Some(input.to_string()));
    }

    #[test]
    fn test_extract_json_array_from_code_block() {
        let input = "Here are the proposals:\n```json\n[{\"title\": \"test\"}]\n```\n";
        assert_eq!(
            extract_json_array(input),
            Some("[{\"title\": \"test\"}]".to_string())
        );
    }
}
