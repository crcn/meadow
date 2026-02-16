use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Movements of Learning ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Movement {
    Supports,
    Deepens,
    RelatesTo,
    Applies,
    Contextualizes,
}

impl Movement {
    pub fn as_relationship_type(&self) -> &'static str {
        match self {
            Movement::Supports => "SUPPORTS",
            Movement::Deepens => "DEEPENS",
            Movement::RelatesTo => "RELATES_TO",
            Movement::Applies => "APPLIES",
            Movement::Contextualizes => "CONTEXTUALIZES",
        }
    }

    pub fn from_relationship_type(s: &str) -> Option<Self> {
        match s {
            "SUPPORTS" => Some(Movement::Supports),
            "DEEPENS" => Some(Movement::Deepens),
            "RELATES_TO" => Some(Movement::RelatesTo),
            "APPLIES" => Some(Movement::Applies),
            "CONTEXTUALIZES" => Some(Movement::Contextualizes),
            _ => None,
        }
    }

    pub fn all() -> &'static [Movement] {
        &[
            Movement::Supports,
            Movement::Deepens,
            Movement::RelatesTo,
            Movement::Applies,
            Movement::Contextualizes,
        ]
    }
}

// ─── Node State (for canvas rendering) ─────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeState {
    Current,
    Visited,
    Proposal,
    TopicRoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EdgeState {
    Traversed,
    Proposal,
}

// ─── Core domain models (Memgraph data) ────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicRoot {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub resource_type: String,
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub channel: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub topic_root_id: Uuid,
    pub resources: Vec<Resource>,
    pub visit_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub movement: Movement,
    pub weight: f64,
    pub traversals: i64,
    pub backups: i64,
}

// ─── Note (from Postgres) ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ─── Assembled graph (returned to frontend) ────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub resources: Vec<Resource>,
    pub notes: Vec<Note>,
    pub state: NodeState,
    pub movement: Option<Movement>,
    pub is_wildcard: bool,
    pub visit_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub movement: Movement,
    pub state: EdgeState,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicGraph {
    pub topic_root: TopicRoot,
    pub current_node_id: Uuid,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}
