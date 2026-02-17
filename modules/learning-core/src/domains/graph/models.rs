use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Movements of Learning ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Movement {
    Deeper,
    Broader,
    Foundation,
    Practice,
    Inspire,
}

impl Movement {
    pub fn as_relationship_type(&self) -> &'static str {
        match self {
            Movement::Deeper => "DEEPER",
            Movement::Broader => "BROADER",
            Movement::Foundation => "FOUNDATION",
            Movement::Practice => "PRACTICE",
            Movement::Inspire => "INSPIRE",
        }
    }

    /// Parse a relationship type string into a Movement.
    /// Accepts both new names and legacy names for backward compatibility
    /// with existing Memgraph edges.
    pub fn from_relationship_type(s: &str) -> Option<Self> {
        match s {
            // New canonical names
            "DEEPER" => Some(Movement::Deeper),
            "BROADER" => Some(Movement::Broader),
            "FOUNDATION" => Some(Movement::Foundation),
            "PRACTICE" => Some(Movement::Practice),
            "INSPIRE" => Some(Movement::Inspire),
            // Legacy names (existing edges in Memgraph)
            "DEEPENS" => Some(Movement::Deeper),
            "SUPPORTS" => Some(Movement::Foundation),
            "RELATES_TO" => Some(Movement::Broader),
            "APPLIES" => Some(Movement::Practice),
            "CONTEXTUALIZES" => Some(Movement::Broader),
            _ => None,
        }
    }

    pub fn all() -> &'static [Movement] {
        &[
            Movement::Deeper,
            Movement::Broader,
            Movement::Foundation,
            Movement::Practice,
            Movement::Inspire,
        ]
    }

    /// All relationship type strings to query, including legacy names
    /// for backward compatibility with existing Memgraph data.
    pub fn all_relationship_types() -> &'static [(&'static str, Movement)] {
        &[
            // New
            ("DEEPER", Movement::Deeper),
            ("BROADER", Movement::Broader),
            ("FOUNDATION", Movement::Foundation),
            ("PRACTICE", Movement::Practice),
            ("INSPIRE", Movement::Inspire),
            // Legacy
            ("DEEPENS", Movement::Deeper),
            ("SUPPORTS", Movement::Foundation),
            ("RELATES_TO", Movement::Broader),
            ("APPLIES", Movement::Practice),
            ("CONTEXTUALIZES", Movement::Broader),
        ]
    }

    /// All Cypher relationship type names that map to this movement
    /// (new + legacy). Used for queries that need to match existing edges.
    pub fn cypher_types(&self) -> &'static [&'static str] {
        match self {
            Movement::Deeper => &["DEEPER", "DEEPENS"],
            Movement::Broader => &["BROADER", "RELATES_TO", "CONTEXTUALIZES"],
            Movement::Foundation => &["FOUNDATION", "SUPPORTS"],
            Movement::Practice => &["PRACTICE", "APPLIES"],
            Movement::Inspire => &["INSPIRE"],
        }
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
    #[serde(default)]
    pub votes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub topic_root_id: Uuid,
    pub resources: Vec<Resource>,
    pub visit_count: i64,
    pub depth: i32,
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
    pub depth: i32,
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
