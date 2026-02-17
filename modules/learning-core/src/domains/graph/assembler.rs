use neo4rs::Graph;
use sqlx::PgPool;
use uuid::Uuid;

use super::models::{EdgeState, GraphEdge, GraphNode, NodeState, Note, TopicGraph};
use super::queries;
use crate::error::Result;

/// Assemble the full visible TopicGraph for a learner on a topic.
///
/// This is THE central function — called by every mutation and query
/// that returns TopicGraph. It gathers data from both Memgraph and
/// Postgres, then tags nodes and edges with their visual state.
pub async fn assemble_topic_graph(
    graph: &Graph,
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
) -> Result<TopicGraph> {
    // 1. Get learner position from Postgres
    let current_node_id = get_learner_position(db, member_id, topic_root_id)
        .await?
        .unwrap_or(topic_root_id);

    // 2. Get visited node IDs from Postgres
    let visited_ids = get_visited_node_ids(db, member_id, topic_root_id).await?;

    // 3. Fetch visible subgraph from Memgraph
    let subgraph = queries::fetch_visible_subgraph(
        graph,
        topic_root_id,
        &visited_ids,
        current_node_id,
    )
    .await?;

    // 4. Fetch notes for all visible nodes
    let node_ids: Vec<Uuid> = subgraph.nodes.iter().map(|n| n.id).collect();
    let notes_map = get_notes_for_nodes(db, &node_ids).await?;

    // 4b. Add the topic root itself as a GraphNode
    let root_graph_node = GraphNode {
        id: subgraph.topic_root.id,
        title: subgraph.topic_root.name.clone(),
        description: subgraph.topic_root.description.clone(),
        resources: vec![],
        notes: notes_map.get(&subgraph.topic_root.id).cloned().unwrap_or_default(),
        state: NodeState::TopicRoot,
        movement: None,
        is_wildcard: false,
        visit_count: 0,
        depth: 0,
    };

    // 5. Tag nodes with state
    let mut nodes: Vec<GraphNode> = vec![root_graph_node];
    nodes.extend(subgraph.nodes.iter().map(|node| {
        let state = if node.id == topic_root_id {
            NodeState::TopicRoot
        } else if node.id == current_node_id {
            NodeState::Current
        } else if visited_ids.contains(&node.id) {
            NodeState::Visited
        } else {
            NodeState::Proposal
        };

        let movement = subgraph
            .edges
            .iter()
            .find(|e| e.target_id == node.id)
            .map(|e| e.movement);

        let notes = notes_map
            .get(&node.id)
            .cloned()
            .unwrap_or_default();

        GraphNode {
            id: node.id,
            title: node.title.clone(),
            description: node.description.clone(),
            resources: node.resources.clone(),
            notes,
            state,
            movement,
            is_wildcard: false,
            visit_count: node.visit_count,
            depth: node.depth,
        }
    }));

    // 6. Tag edges with state
    let edges: Vec<GraphEdge> = subgraph
        .edges
        .iter()
        .map(|edge| {
            let source_visited = visited_ids.contains(&edge.source_id)
                || edge.source_id == current_node_id
                || edge.source_id == topic_root_id;
            let target_visited = visited_ids.contains(&edge.target_id)
                || edge.target_id == current_node_id;

            let state = if source_visited && target_visited {
                EdgeState::Traversed
            } else {
                EdgeState::Proposal
            };

            GraphEdge {
                id: format!("{}-{}", edge.source_id, edge.target_id),
                source_id: edge.source_id,
                target_id: edge.target_id,
                movement: edge.movement,
                state,
                weight: edge.weight,
            }
        })
        .collect();

    // 7. Sort proposal nodes: highest incoming edge weight first
    nodes.sort_by(|a, b| {
        if a.state == NodeState::Proposal && b.state == NodeState::Proposal {
            let w_a = edges.iter().find(|e| e.target_id == a.id).map(|e| e.weight).unwrap_or(0.0);
            let w_b = edges.iter().find(|e| e.target_id == b.id).map(|e| e.weight).unwrap_or(0.0);
            w_b.partial_cmp(&w_a).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            std::cmp::Ordering::Equal
        }
    });

    Ok(TopicGraph {
        topic_root: subgraph.topic_root,
        current_node_id,
        nodes,
        edges,
    })
}

// ─── Postgres helpers (runtime queries) ────────────────────────────

async fn get_learner_position(
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
) -> Result<Option<Uuid>> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "SELECT current_node_id FROM learner_position WHERE member_id = $1 AND topic_root_id = $2"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|(id,)| id))
}

async fn get_visited_node_ids(
    db: &PgPool,
    member_id: Uuid,
    topic_root_id: Uuid,
) -> Result<Vec<Uuid>> {
    let rows: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT DISTINCT node_id FROM traversal_history WHERE member_id = $1 AND topic_root_id = $2"
    )
    .bind(member_id)
    .bind(topic_root_id)
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|(id,)| id).collect())
}

async fn get_notes_for_nodes(
    db: &PgPool,
    node_ids: &[Uuid],
) -> Result<std::collections::HashMap<Uuid, Vec<Note>>> {
    let rows: Vec<NoteRow> = sqlx::query_as(
        "SELECT id, node_id, body, created_at FROM node_notes WHERE node_id = ANY($1) ORDER BY created_at ASC"
    )
    .bind(node_ids)
    .fetch_all(db)
    .await?;

    let mut map: std::collections::HashMap<Uuid, Vec<Note>> = std::collections::HashMap::new();
    for row in rows {
        map.entry(row.node_id).or_default().push(Note {
            id: row.id,
            body: row.body,
            created_at: row.created_at,
        });
    }

    Ok(map)
}

#[derive(sqlx::FromRow)]
struct NoteRow {
    id: Uuid,
    node_id: Uuid,
    body: String,
    created_at: chrono::DateTime<chrono::Utc>,
}
