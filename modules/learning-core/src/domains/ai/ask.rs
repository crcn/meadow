use ai_client::traits::{Agent, PromptBuilder};

use super::prompts::ASK_ABOUT_NODE_PROMPT;
use crate::domains::graph::models::GraphNode;
use crate::error::{Error, Result};

/// Answer a learner's question about a node.
/// Provides the node context and learning path to the AI.
pub async fn ask_about_node<A: Agent>(
    ai_agent: &A,
    node: &GraphNode,
    path: &[String],
    question: &str,
) -> Result<String> {
    tracing::info!(
        node = %node.title,
        question = %question,
        path_len = path.len(),
        "Asking AI about node"
    );

    let context = format!(
        "Current node: {} — {}\nResources: {}\nLearning path: {}\n\nLearner's question: {}",
        node.title,
        node.description,
        node.resources
            .iter()
            .map(|r| format!("{}: {}", r.resource_type, r.title))
            .collect::<Vec<_>>()
            .join(", "),
        if path.is_empty() {
            "Just started".to_string()
        } else {
            path.join(" → ")
        },
        question
    );

    let start = std::time::Instant::now();

    let answer = ai_agent
        .prompt(&context)
        .preamble(ASK_ABOUT_NODE_PROMPT)
        .send()
        .await
        .map_err(|e| Error::Ai(e.to_string()))?;

    tracing::info!(
        answer_len = answer.len(),
        elapsed_ms = start.elapsed().as_millis() as u64,
        "AI answer received"
    );

    Ok(answer)
}
