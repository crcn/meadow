use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use learning_core::types::*;
use learning_core::ServerDeps;
use learning_domains::curriculum::models::{CurriculumRow, EdgeRow, NodeRow, VideoRow};
use uuid::Uuid;

pub fn router(deps: ServerDeps) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/curricula", get(list_curricula))
        .route("/api/curricula/{id}", get(get_curriculum))
        .route("/api/nodes/{id}", get(get_node))
        .with_state(deps)
}

async fn health() -> &'static str {
    "ok"
}

async fn list_curricula(
    State(deps): State<ServerDeps>,
) -> Result<Json<Vec<CurriculumSummary>>, StatusCode> {
    let rows = CurriculumRow::list_all(&deps.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows.iter().map(|r| r.to_summary()).collect()))
}

async fn get_curriculum(
    State(deps): State<ServerDeps>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let curriculum = CurriculumRow::find_by_id(&deps.db_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let chapters = NodeRow::find_by_curriculum(&deps.db_pool, id, Some(0))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "curriculum": curriculum.to_summary(),
        "chapters": chapters.iter().map(|n| n.to_summary()).collect::<Vec<_>>(),
    })))
}

async fn get_node(
    State(deps): State<ServerDeps>,
    Path(id): Path<Uuid>,
) -> Result<Json<NodeDetail>, StatusCode> {
    let node = NodeRow::find_by_id(&deps.db_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let children = NodeRow::find_children(&deps.db_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let edges_rows = EdgeRow::find_from_node(&deps.db_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let videos = VideoRow::find_by_node(&deps.db_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Resolve edge target nodes
    let mut edges = NodeEdges::default();
    for edge in &edges_rows {
        if let Ok(Some(target)) = NodeRow::find_by_id(&deps.db_pool, edge.to_node).await {
            let summary = target.to_summary();
            match edge.edge_type {
                EdgeType::Next => edges.next.push(summary),
                EdgeType::Deeper => edges.deeper.push(summary),
                EdgeType::Sibling => edges.siblings.push(summary),
            }
        }
    }

    Ok(Json(NodeDetail {
        id: node.id,
        curriculum_id: node.curriculum_id,
        title: node.title,
        summary: node.summary,
        content: node.content,
        node_type: node.node_type,
        status: node.status,
        depth: node.depth,
        position: node.position,
        parent_node_id: node.parent_node_id,
        children: children.iter().map(|n| n.to_summary()).collect(),
        edges,
        videos: videos.iter().map(|v| v.to_summary()).collect(),
    }))
}
