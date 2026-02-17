use ai_client::traits::{Agent, Message, PromptBuilder};
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
        let youtube_id = r.youtube_id.or_else(|| extract_youtube_id(r.url.as_deref()));

        Resource {
            resource_type: r.resource_type,
            youtube_id,
            url: r.url,
            title: r.title,
            channel: r.channel,
            reason: r.reason,
            votes: 0,
        }
    }
}

/// Extract a YouTube video ID from a URL like:
///   https://www.youtube.com/watch?v=abc123XYZ_-
///   https://youtu.be/abc123XYZ_-
fn extract_youtube_id(url: Option<&str>) -> Option<String> {
    let url = url?;
    // youtube.com/watch?v=ID
    if let Some(pos) = url.find("v=") {
        let after = &url[pos + 2..];
        let id: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_').collect();
        if id.len() == 11 { return Some(id); }
    }
    // youtu.be/ID
    if let Some(pos) = url.find("youtu.be/") {
        let after = &url[pos + 9..];
        let id: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_').collect();
        if id.len() == 11 { return Some(id); }
    }
    None
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

    let mut agent = agent
        .clone()
        .tool(TavilySearchTool::new(tavily_api_key.to_string()))
        .tool(ExistingNodesTool::new(memgraph));

    if !youtube_api_key.is_empty() {
        agent = agent
            .tool(YoutubeSearchTool::new(youtube_api_key.to_string()))
            .tool(YoutubeDetailsTool::new(youtube_api_key.to_string()));
    }

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

    // Parse JSON array from the response — retry once if the AI didn't return JSON
    let proposals = match parse_proposals(&response) {
        Ok(p) => p,
        Err(_) => {
            tracing::warn!("First parse failed, asking AI to reformat as JSON");
            let retry_response = agent
                .prompt("Your previous response did not contain a valid JSON array. Please reformat your findings as a JSON array exactly matching this schema:\n[{\"title\": \"...\", \"description\": \"...\", \"movement\": \"SUPPORTS|DEEPENS|RELATES_TO|APPLIES|CONTEXTUALIZES\", \"resources\": [{\"type\": \"...\", \"url\": \"...\", \"title\": \"...\", \"channel\": null, \"reason\": \"...\"}]}]\nReturn ONLY the JSON array, no other text.")
                .preamble(system_prompt)
                .messages(vec![
                    Message::user(user_prompt.to_string()),
                    Message::assistant(response.clone()),
                ])
                .send()
                .await
                .map_err(|e| Error::Ai(e.to_string()))?;

            tracing::info!(
                retry_response_len = retry_response.len(),
                "AI reformat response received"
            );
            parse_proposals(&retry_response)?
        }
    };

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
    let json_str = match extract_json_array(response) {
        Some(s) => s,
        None => {
            tracing::error!(
                response_len = response.len(),
                response_preview = %&response[..response.len().min(500)],
                "No JSON array found in agent response"
            );
            return Err(Error::Ai("No JSON array found in agent response".into()));
        }
    };

    serde_json::from_str(&json_str).map_err(|e| {
        tracing::error!(
            json_preview = %&json_str[..json_str.len().min(500)],
            error = %e,
            "Failed to parse proposals JSON"
        );
        Error::Ai(format!("Failed to parse proposals: {}", e))
    })
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
            let block = after_marker[..end].trim();
            // Handle both arrays and objects within code blocks
            if block.starts_with('[') {
                return Some(block.to_string());
            }
            if block.starts_with('{') {
                if let Some(arr) = extract_array_from_object(block) {
                    return Some(arr);
                }
            }
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
            if block.starts_with('{') {
                if let Some(arr) = extract_array_from_object(block) {
                    return Some(arr);
                }
            }
        }
    }

    // Try to find first [ and last ]
    if let (Some(start), Some(end)) = (text.find('['), text.rfind(']')) {
        if end > start {
            return Some(text[start..=end].to_string());
        }
    }

    // Last resort: the AI returned a JSON object wrapping the array
    // e.g. {"proposals": [...]} or {"nodes": [...]}
    if text.contains('{') {
        // Find the outermost JSON object
        let obj_start = text.find('{')?;
        let obj_end = text.rfind('}')?;
        if obj_end > obj_start {
            let obj_text = &text[obj_start..=obj_end];
            if let Some(arr) = extract_array_from_object(obj_text) {
                return Some(arr);
            }
        }
    }

    None
}

/// Given a JSON object string, extract the first array value from any key.
fn extract_array_from_object(text: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let obj = value.as_object()?;
    for (_key, val) in obj {
        if val.is_array() {
            return Some(val.to_string());
        }
    }
    None
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

    #[test]
    fn test_extract_youtube_id_watch_url() {
        let url = "https://www.youtube.com/watch?v=juxycZTFZ6M";
        assert_eq!(extract_youtube_id(Some(url)), Some("juxycZTFZ6M".to_string()));
    }

    #[test]
    fn test_extract_youtube_id_short_url() {
        let url = "https://youtu.be/juxycZTFZ6M";
        assert_eq!(extract_youtube_id(Some(url)), Some("juxycZTFZ6M".to_string()));
    }

    #[test]
    fn test_extract_youtube_id_playlist_returns_none() {
        let url = "https://www.youtube.com/playlist?list=PLRA7uxKdQNb0";
        assert_eq!(extract_youtube_id(Some(url)), None);
    }

    #[test]
    fn test_extract_youtube_id_none_url() {
        assert_eq!(extract_youtube_id(None), None);
    }
}
