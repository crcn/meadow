use ai_client::traits::{Agent, PromptBuilder};
use ai_client::OpenAi;
use neo4rs::Graph;
use schemars::JsonSchema;
use std::sync::Arc;

use super::tools::{
    existing_nodes::ExistingNodesTool, tavily::TavilySearchTool,
    youtube_details::YoutubeDetailsTool, youtube_search::YoutubeSearchTool,
};
use crate::domains::graph::models::Resource;
use crate::error::{Error, Result};

/// Wrapper for structured output extraction (OpenAI requires a top-level object, not an array).
#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct ProposalResponse {
    pub proposals: Vec<ProposedNode>,
}

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct ProposedNode {
    pub title: String,
    pub description: String,
    /// One of: DEEPER, BROADER, FOUNDATION, PRACTICE, INSPIRE
    pub movement: String,
    pub resources: Vec<ProposedResource>,
}

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct ProposedResource {
    /// The resource type, e.g. "video", "article", "course"
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
///
/// Two-phase approach:
/// 1. Multi-turn agent loop with tools (search, YouTube, existing nodes) — returns freeform text
/// 2. Structured output extraction — guarantees valid typed JSON via OpenAI's json_schema mode
pub async fn investigate(
    agent: &OpenAi,
    memgraph: Arc<Graph>,
    tavily_api_key: &str,
    youtube_api_key: &str,
    system_prompt: &str,
    user_prompt: &str,
    max_turns: usize,
    steered: bool,
) -> Result<Vec<ProposedNode>> {
    tracing::info!(
        max_turns = max_turns,
        prompt_len = user_prompt.len(),
        "Starting AI investigation"
    );

    let mut tool_agent = agent
        .clone()
        .tool(TavilySearchTool::new(tavily_api_key.to_string()))
        .tool(ExistingNodesTool::new(memgraph));

    if !youtube_api_key.is_empty() {
        tool_agent = tool_agent
            .tool(YoutubeSearchTool::new(youtube_api_key.to_string()))
            .tool(YoutubeDetailsTool::new(youtube_api_key.to_string()));
    }

    let start = std::time::Instant::now();

    // Phase 1: Multi-turn tool-calling investigation
    let research = tool_agent
        .prompt(user_prompt)
        .preamble(system_prompt)
        .multi_turn(max_turns)
        .send()
        .await
        .map_err(|e| Error::Ai(e.to_string()))?;

    tracing::info!(
        research_len = research.len(),
        elapsed_ms = start.elapsed().as_millis() as u64,
        "AI investigation complete, extracting structured proposals"
    );

    // Phase 2: Structured output extraction — guaranteed valid JSON
    let extraction_prompt = if steered {
        format!(
            "Based on the following research findings, extract exactly 3 learning node proposals \
             matching the learner's requested direction.\n\n\
             Research findings:\n{}\n\n\
             Pick the BEST-FIT movement type for each proposal (DEEPER, BROADER, FOUNDATION, PRACTICE, or INSPIRE). \
             Any combination is allowed — choose what fits the learner's request.\n\n\
             Each proposal needs: title, description, movement (DEEPER/BROADER/FOUNDATION/PRACTICE/INSPIRE), and resources.",
            research
        )
    } else {
        format!(
            "Based on the following research findings, extract exactly 3 learning node proposals.\n\n\
             Research findings:\n{}\n\n\
             Extract exactly 3 proposals with these movement types:\n\
             - 1x DEEPER (must be harder/more advanced than the current node)\n\
             - 1x PRACTICE (a hands-on exercise, project, or challenge)\n\
             - 1x BROADER, FOUNDATION, or INSPIRE (lateral exploration, prerequisite backfill, or inspirational content)\n\n\
             Each proposal needs: title, description, movement (DEEPER/BROADER/FOUNDATION/PRACTICE/INSPIRE), and resources.",
            research
        )
    };

    let response: ProposalResponse = agent
        .extract(agent.model(), system_prompt, &extraction_prompt)
        .await
        .map_err(|e| Error::Ai(e.to_string()))?;

    tracing::info!(
        elapsed_ms = start.elapsed().as_millis() as u64,
        proposals = response.proposals.len(),
        "Structured extraction complete"
    );

    for (i, p) in response.proposals.iter().enumerate() {
        tracing::info!(
            index = i,
            title = %p.title,
            movement = %p.movement,
            resources = p.resources.len(),
            "Parsed proposal"
        );
    }

    Ok(response.proposals)
}

#[cfg(test)]
mod tests {
    use super::*;

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
