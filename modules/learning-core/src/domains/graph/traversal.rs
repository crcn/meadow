use neo4rs::Graph;
use sqlx::PgPool;
use uuid::Uuid;

use super::models::{Movement, TopicGraph};
use super::{assembler, queries};
use crate::error::{Error, Result};

/// Traverse from one node to another.
pub async fn traverse(
    graph: &Graph,
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
    from_node_id: Uuid,
    to_node_id: Uuid,
    movement: Movement,
) -> Result<TopicGraph> {
    // Validate: target node must exist and belong to this topic
    let target_node = queries::get_node(graph, to_node_id)
        .await?
        .ok_or_else(|| Error::Validation("Target node does not exist".into()))?;

    if target_node.topic_root_id != topic_root_id {
        return Err(Error::Validation("Target node does not belong to this topic".into()));
    }

    // Validate: an edge of this movement type must exist between from and to
    let edge_ok = queries::edge_exists(graph, from_node_id, to_node_id, movement).await?;
    if !edge_ok {
        return Err(Error::Validation(format!(
            "No {} edge exists from the current node to the target node",
            movement.as_relationship_type()
        )));
    }

    // Update edge weight in Memgraph
    queries::increment_traversal(graph, from_node_id, to_node_id, movement).await?;

    // Record in Postgres
    sqlx::query(
        "INSERT INTO traversal_history (member_id, topic_root_id, node_id, previous_node_id, movement_type, is_backtrack) VALUES ($1, $2, $3, $4, $5, false)"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .bind(to_node_id)
    .bind(from_node_id)
    .bind(movement.as_relationship_type())
    .execute(db)
    .await?;

    // Update learner position (upsert)
    sqlx::query(
        "INSERT INTO learner_position (member_id, topic_root_id, current_node_id) VALUES ($1, $2, $3) ON CONFLICT (member_id, topic_root_id) DO UPDATE SET current_node_id = $3, updated_at = now()"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .bind(to_node_id)
    .execute(db)
    .await?;

    assembler::assemble_topic_graph(graph, db, member_id, topic_root_id).await
}

/// Back up to the previous node. Downvotes the abandoned edge.
pub async fn back_up(
    graph: &Graph,
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
) -> Result<TopicGraph> {
    // Find current position
    let current: Uuid = sqlx::query_scalar(
        "SELECT current_node_id FROM learner_position WHERE member_id = $1 AND topic_root_id = $2"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| Error::NotFound("No active position".into()))?;

    // Cannot back up from the topic root
    if current == topic_root_id {
        return Err(Error::Validation("Already at the topic root — cannot back up further".into()));
    }

    // Get the previous node from most recent non-backtrack traversal
    let previous: Uuid = sqlx::query_scalar(
        "SELECT previous_node_id FROM traversal_history WHERE member_id = $1 AND topic_root_id = $2 AND node_id = $3 AND is_backtrack = false AND previous_node_id IS NOT NULL ORDER BY created_at DESC LIMIT 1"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .bind(current)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| Error::Validation("No previous node to back up to".into()))?;

    // Find the movement type of the edge we're backing away from
    let movement_str: Option<String> = sqlx::query_scalar(
        "SELECT movement_type FROM traversal_history WHERE member_id = $1 AND topic_root_id = $2 AND node_id = $3 AND is_backtrack = false ORDER BY created_at DESC LIMIT 1"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .bind(current)
    .fetch_optional(db)
    .await?;

    // Downvote the edge in Memgraph
    if let Some(ref movement_str) = movement_str {
        if let Some(movement) = Movement::from_relationship_type(movement_str) {
            queries::increment_backup(graph, previous, current, movement).await?;
        }
    }

    // Record backtrack
    sqlx::query(
        "INSERT INTO traversal_history (member_id, topic_root_id, node_id, previous_node_id, movement_type, is_backtrack) VALUES ($1, $2, $3, $4, $5, true)"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .bind(previous)
    .bind(current)
    .bind(movement_str.as_deref())
    .execute(db)
    .await?;

    // Update position
    sqlx::query(
        "UPDATE learner_position SET current_node_id = $3, updated_at = now() WHERE member_id = $1 AND topic_root_id = $2"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .bind(previous)
    .execute(db)
    .await?;

    assembler::assemble_topic_graph(graph, db, member_id, topic_root_id).await
}
