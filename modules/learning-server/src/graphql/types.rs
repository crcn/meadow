use async_graphql::{Enum, SimpleObject, ID};
use learning_core::domains::graph::models as dm;

// ─── Enums ─────────────────────────────────────────────────────────

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Movement {
    Supports,
    Deepens,
    RelatesTo,
    Applies,
    Contextualizes,
}

impl From<dm::Movement> for Movement {
    fn from(m: dm::Movement) -> Self {
        match m {
            dm::Movement::Supports => Movement::Supports,
            dm::Movement::Deepens => Movement::Deepens,
            dm::Movement::RelatesTo => Movement::RelatesTo,
            dm::Movement::Applies => Movement::Applies,
            dm::Movement::Contextualizes => Movement::Contextualizes,
        }
    }
}

impl From<Movement> for dm::Movement {
    fn from(m: Movement) -> Self {
        match m {
            Movement::Supports => dm::Movement::Supports,
            Movement::Deepens => dm::Movement::Deepens,
            Movement::RelatesTo => dm::Movement::RelatesTo,
            Movement::Applies => dm::Movement::Applies,
            Movement::Contextualizes => dm::Movement::Contextualizes,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum NodeState {
    Current,
    Visited,
    Proposal,
    TopicRoot,
}

impl From<dm::NodeState> for NodeState {
    fn from(s: dm::NodeState) -> Self {
        match s {
            dm::NodeState::Current => NodeState::Current,
            dm::NodeState::Visited => NodeState::Visited,
            dm::NodeState::Proposal => NodeState::Proposal,
            dm::NodeState::TopicRoot => NodeState::TopicRoot,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum EdgeState {
    Traversed,
    Proposal,
}

impl From<dm::EdgeState> for EdgeState {
    fn from(s: dm::EdgeState) -> Self {
        match s {
            dm::EdgeState::Traversed => EdgeState::Traversed,
            dm::EdgeState::Proposal => EdgeState::Proposal,
        }
    }
}

// ─── Object types ──────────────────────────────────────────────────

#[derive(SimpleObject)]
pub struct TopicRoot {
    pub id: ID,
    pub name: String,
    pub description: String,
}

impl From<dm::TopicRoot> for TopicRoot {
    fn from(t: dm::TopicRoot) -> Self {
        TopicRoot {
            id: t.id.to_string().into(),
            name: t.name,
            description: t.description,
        }
    }
}

#[derive(SimpleObject)]
pub struct Resource {
    #[graphql(name = "type")]
    pub resource_type: String,
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub channel: Option<String>,
    pub reason: String,
    pub votes: i32,
}

impl From<dm::Resource> for Resource {
    fn from(r: dm::Resource) -> Self {
        Resource {
            resource_type: r.resource_type,
            youtube_id: r.youtube_id,
            url: r.url,
            title: r.title,
            channel: r.channel,
            reason: r.reason,
            votes: r.votes,
        }
    }
}

#[derive(SimpleObject)]
pub struct Note {
    pub id: ID,
    pub body: String,
    pub created_at: String,
}

impl From<dm::Note> for Note {
    fn from(n: dm::Note) -> Self {
        Note {
            id: n.id.to_string().into(),
            body: n.body,
            created_at: n.created_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct GraphNode {
    pub id: ID,
    pub title: String,
    pub description: String,
    pub resources: Vec<Resource>,
    pub notes: Vec<Note>,
    pub state: NodeState,
    pub movement: Option<Movement>,
    pub is_wildcard: bool,
    pub visit_count: i32,
}

impl From<dm::GraphNode> for GraphNode {
    fn from(n: dm::GraphNode) -> Self {
        GraphNode {
            id: n.id.to_string().into(),
            title: n.title,
            description: n.description,
            resources: n.resources.into_iter().map(Into::into).collect(),
            notes: n.notes.into_iter().map(Into::into).collect(),
            state: n.state.into(),
            movement: n.movement.map(Into::into),
            is_wildcard: n.is_wildcard,
            visit_count: n.visit_count as i32,
        }
    }
}

#[derive(SimpleObject)]
pub struct GraphEdge {
    pub id: ID,
    pub source_id: ID,
    pub target_id: ID,
    pub movement: Movement,
    pub state: EdgeState,
    pub weight: f64,
}

impl From<dm::GraphEdge> for GraphEdge {
    fn from(e: dm::GraphEdge) -> Self {
        GraphEdge {
            id: e.id.into(),
            source_id: e.source_id.to_string().into(),
            target_id: e.target_id.to_string().into(),
            movement: e.movement.into(),
            state: e.state.into(),
            weight: e.weight,
        }
    }
}

#[derive(SimpleObject)]
pub struct TopicGraph {
    pub topic_root: TopicRoot,
    pub current_node_id: ID,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl From<dm::TopicGraph> for TopicGraph {
    fn from(g: dm::TopicGraph) -> Self {
        TopicGraph {
            topic_root: g.topic_root.into(),
            current_node_id: g.current_node_id.to_string().into(),
            nodes: g.nodes.into_iter().map(Into::into).collect(),
            edges: g.edges.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(SimpleObject)]
pub struct SessionTopic {
    pub topic_root: TopicRoot,
    pub current_node_id: ID,
    pub last_active: String,
}

#[derive(SimpleObject)]
pub struct AuthResult {
    pub success: bool,
}

#[derive(SimpleObject)]
pub struct VerifyResult {
    pub member_id: ID,
    pub token: String,
}

#[derive(SimpleObject)]
pub struct AiResponse {
    pub answer: String,
}
