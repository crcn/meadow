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
    pub summary: String,
    pub nodes: Vec<ExistingNode>,
}

#[derive(Debug, Serialize)]
pub struct ExistingNode {
    pub id: String,
    pub title: String,
    pub description: String,
    pub depth: i64,
    pub outgoing: Vec<ExistingEdge>,
}

#[derive(Debug, Serialize)]
pub struct ExistingEdge {
    pub target_title: String,
    pub movement: String,
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
        // Fetch nodes with their outgoing edges
        let mut result = self
            .graph
            .execute(
                neo4rs::query(
                    "MATCH (n:Node {topic_root_id: $topic_root_id}) \
                     OPTIONAL MATCH (n)-[r]->(m) \
                     RETURN n.id AS id, n.title AS title, n.description AS description, \
                            n.depth AS depth, \
                            collect(CASE WHEN m IS NOT NULL THEN {target_title: m.title, movement: type(r)} ELSE NULL END) AS edges",
                )
                .param("topic_root_id", args.topic_root_id.clone()),
            )
            .await
            .map_err(|e| ExistingNodesError::Graph(e.to_string()))?;

        let mut nodes = Vec::new();
        while let Some(row) = result
            .next()
            .await
            .map_err(|e| ExistingNodesError::Graph(e.to_string()))?
        {
            let depth: i64 = row.get::<i64>("depth").unwrap_or(0);

            // Parse edge list from the collected maps
            let edges_raw: Vec<std::collections::HashMap<String, String>> =
                row.get("edges").unwrap_or_default();
            let outgoing: Vec<ExistingEdge> = edges_raw
                .into_iter()
                .filter(|e| !e.is_empty())
                .map(|e| ExistingEdge {
                    target_title: e.get("target_title").cloned().unwrap_or_default(),
                    movement: e.get("movement").cloned().unwrap_or_default(),
                })
                .collect();

            nodes.push(ExistingNode {
                id: row.get("id").unwrap_or_default(),
                title: row.get("title").unwrap_or_default(),
                description: row.get("description").unwrap_or_default(),
                depth,
                outgoing,
            });
        }

        let summary = format!(
            "Graph has {} nodes across {} edges.",
            nodes.len(),
            nodes.iter().map(|n| n.outgoing.len()).sum::<usize>()
        );

        Ok(ExistingNodesResult { summary, nodes })
    }
}
