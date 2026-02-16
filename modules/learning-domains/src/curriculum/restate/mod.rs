use learning_core::types::*;
use learning_core::ServerDeps;
use restate_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::activities::generate_curriculum::generate_curriculum_outline;
use super::models::{CurriculumRow, EdgeRow, NodeRow};

// ── Request/Response types for Restate handlers ─────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCurriculumRequest {
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCurriculumResponse {
    pub curriculum_id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandNodeRequest {
    pub action: ExpandAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandNodeResponse {
    pub created_nodes: Vec<Uuid>,
}

// ── CurriculumWorkflow ──────────────────────────────────────────────

#[restate_sdk::workflow]
pub trait CurriculumWorkflow {
    async fn run(req: CreateCurriculumRequest) -> Result<CreateCurriculumResponse, HandlerError>;
    #[shared]
    async fn status() -> Result<String, HandlerError>;
}

pub struct CurriculumWorkflowImpl {
    deps: ServerDeps,
}

impl CurriculumWorkflowImpl {
    pub fn with_deps(deps: ServerDeps) -> Self {
        Self { deps }
    }
}

impl CurriculumWorkflow for CurriculumWorkflowImpl {
    async fn run(
        &self,
        _ctx: WorkflowContext<'_>,
        req: CreateCurriculumRequest,
    ) -> Result<CreateCurriculumResponse, HandlerError> {
        let deps = &self.deps;
        let topic = &req.topic;

        tracing::info!(topic, "Generating curriculum");

        // Step 1: Call AI to generate curriculum outline
        let outline = generate_curriculum_outline(deps, topic)
            .await
            .map_err(|e| HandlerError::from(e.to_string()))?;

        // Step 2: Insert curriculum into database
        let curriculum = CurriculumRow::insert(
            &deps.db_pool,
            &outline.title,
            &outline.description,
            topic,
        )
        .await
        .map_err(|e| HandlerError::from(e.to_string()))?;

        // Step 3: Insert chapter nodes
        let mut prev_node_id: Option<Uuid> = None;
        for (i, chapter) in outline.chapters.iter().enumerate() {
            let node = NodeRow::insert(
                &deps.db_pool,
                curriculum.id,
                &chapter.title,
                &chapter.summary,
                &chapter.content,
                NodeType::Chapter,
                0,
                i as i32,
                None,
            )
            .await
            .map_err(|e| HandlerError::from(e.to_string()))?;

            // Create "next" edges between consecutive chapters
            if let Some(prev_id) = prev_node_id {
                EdgeRow::insert(
                    &deps.db_pool,
                    prev_id,
                    node.id,
                    EdgeType::Next,
                    0,
                )
                .await
                .map_err(|e| HandlerError::from(e.to_string()))?;
            }

            prev_node_id = Some(node.id);
        }

        tracing::info!(
            curriculum_id = %curriculum.id,
            title = %curriculum.title,
            chapters = outline.chapters.len(),
            "Curriculum generated"
        );

        Ok(CreateCurriculumResponse {
            curriculum_id: curriculum.id,
            title: curriculum.title,
        })
    }

    async fn status(
        &self,
        _ctx: SharedWorkflowContext<'_>,
    ) -> Result<String, HandlerError> {
        Ok("running".to_string())
    }
}

// ── NodeObject (virtual object keyed by node_id) ────────────────────

#[restate_sdk::object]
pub trait NodeObject {
    async fn expand(req: ExpandNodeRequest) -> Result<ExpandNodeResponse, HandlerError>;
    #[shared]
    async fn get_status() -> Result<String, HandlerError>;
}

pub struct NodeObjectImpl {
    deps: ServerDeps,
}

impl NodeObjectImpl {
    pub fn with_deps(deps: ServerDeps) -> Self {
        Self { deps }
    }
}

impl NodeObject for NodeObjectImpl {
    async fn expand(
        &self,
        ctx: ObjectContext<'_>,
        _req: ExpandNodeRequest,
    ) -> Result<ExpandNodeResponse, HandlerError> {
        let node_id: Uuid = ctx
            .key()
            .parse()
            .map_err(|e: uuid::Error| HandlerError::from(e.to_string()))?;

        // Check if node exists
        let _node = NodeRow::find_by_id(&self.deps.db_pool, node_id)
            .await
            .map_err(|e| HandlerError::from(e.to_string()))?
            .ok_or_else(|| HandlerError::from(format!("Node not found: {}", node_id)))?;

        // TODO: Phase 2 — implement expand_deeper, generate_siblings, generate_next
        // For now, return empty
        tracing::info!(node_id = %node_id, "Node expand requested (stub)");

        Ok(ExpandNodeResponse {
            created_nodes: vec![],
        })
    }

    async fn get_status(
        &self,
        ctx: SharedObjectContext<'_>,
    ) -> Result<String, HandlerError> {
        let node_id: Uuid = ctx
            .key()
            .parse()
            .map_err(|e: uuid::Error| HandlerError::from(e.to_string()))?;

        let node = NodeRow::find_by_id(&self.deps.db_pool, node_id)
            .await
            .map_err(|e| HandlerError::from(e.to_string()))?;

        match node {
            Some(n) => Ok(format!("{:?}", n.status)),
            None => Ok("not_found".to_string()),
        }
    }
}

// ── PrefetchService (stub for Phase 3) ──────────────────────────────

#[restate_sdk::service]
pub trait PrefetchService {
    async fn prefetch_around_node(node_id: String) -> Result<(), HandlerError>;
}

pub struct PrefetchServiceImpl {
    _deps: ServerDeps,
}

impl PrefetchServiceImpl {
    pub fn with_deps(deps: ServerDeps) -> Self {
        Self { _deps: deps }
    }
}

impl PrefetchService for PrefetchServiceImpl {
    async fn prefetch_around_node(
        &self,
        _ctx: Context<'_>,
        node_id: String,
    ) -> Result<(), HandlerError> {
        tracing::info!(node_id, "Prefetch requested (stub — Phase 3)");
        Ok(())
    }
}
