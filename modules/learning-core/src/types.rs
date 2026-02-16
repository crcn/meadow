use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type NodeId = Uuid;
pub type CurriculumId = Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "edge_type", rename_all = "lowercase")]
pub enum EdgeType {
    Next,
    Deeper,
    Sibling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "node_type", rename_all = "lowercase")]
pub enum NodeType {
    Chapter,
    Page,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "node_status", rename_all = "lowercase")]
pub enum NodeStatus {
    Ready,
    Generating,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpandAction {
    Deeper,
    Siblings,
    Next,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurriculumSummary {
    pub id: CurriculumId,
    pub title: String,
    pub topic: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSummary {
    pub id: NodeId,
    pub title: String,
    pub summary: String,
    pub node_type: NodeType,
    pub status: NodeStatus,
    pub depth: i32,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDetail {
    pub id: NodeId,
    pub curriculum_id: CurriculumId,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub node_type: NodeType,
    pub status: NodeStatus,
    pub depth: i32,
    pub position: i32,
    pub parent_node_id: Option<NodeId>,
    pub children: Vec<NodeSummary>,
    pub edges: NodeEdges,
    pub videos: Vec<VideoSummary>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeEdges {
    pub next: Vec<NodeSummary>,
    pub deeper: Vec<NodeSummary>,
    pub siblings: Vec<NodeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSummary {
    pub id: Uuid,
    pub youtube_id: String,
    pub title: String,
    pub channel_name: String,
    pub thumbnail_url: String,
    pub duration_secs: i32,
    pub rank: i32,
    pub relevance_score: f32,
    pub ai_rationale: String,
}

/// AI-generated curriculum outline used as intermediate representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurriculumOutline {
    pub title: String,
    pub description: String,
    pub chapters: Vec<ChapterOutline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterOutline {
    pub title: String,
    pub summary: String,
    pub content: String,
}
