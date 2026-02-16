use ai_client::tool::{Tool, ToolDefinition};
use async_trait::async_trait;
use neo4rs::Graph;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct ExistingNodesTool {
    graph: Arc<Graph>,
}

impl std::fmt::Debug for ExistingNodesTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExistingNodesTool").finish()
    }
}

impl ExistingNodesTool {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph }
    }
}

#[derive(Debug, Deserialize)]
pub struct ExistingNodesArgs {
    pub topic_root_id: String,
}

#[derive(Debug, Serialize)]
pub struct ExistingNodesResult {
    pub nodes: Vec<ExistingNode>,
}

#[derive(Debug, Serialize)]
pub struct ExistingNode {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ExistingNodesError {
    #[error("graph error: {0}")]
    Graph(String),
}

#[async_trait]
impl Tool for ExistingNodesTool {
    const NAME: &'static str = "existing_nodes";
    type Error = ExistingNodesError;
    type Args = ExistingNodesArgs;
    type Output = ExistingNodesResult;

    async fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get all existing nodes in a topic graph. Use this to avoid creating duplicate concepts.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic_root_id": {
                        "type": "string",
                        "description": "The topic root ID to search within"
                    }
                },
                "required": ["topic_root_id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut result = self
            .graph
            .execute(
                neo4rs::query(
                    "MATCH (n:Node {topic_root_id: $topic_root_id}) RETURN n.id AS id, n.title AS title, n.description AS description",
                )
                .param("topic_root_id", args.topic_root_id),
            )
            .await
            .map_err(|e| ExistingNodesError::Graph(e.to_string()))?;

        let mut nodes = Vec::new();
        while let Some(row) = result
            .next()
            .await
            .map_err(|e| ExistingNodesError::Graph(e.to_string()))?
        {
            nodes.push(ExistingNode {
                id: row.get("id").unwrap_or_default(),
                title: row.get("title").unwrap_or_default(),
                description: row.get("description").unwrap_or_default(),
            });
        }

        Ok(ExistingNodesResult { nodes })
    }
}
