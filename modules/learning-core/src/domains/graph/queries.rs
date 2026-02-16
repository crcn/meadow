use neo4rs::{query, Graph};
use uuid::Uuid;

use super::models::{Edge, Movement, Node, Resource, TopicRoot};
use crate::error::{Error, Result};

// ─── TopicRoot operations ──────────────────────────────────────────

pub async fn create_topic_root(
    graph: &Graph,
    id: Uuid,
    name: &str,
    description: &str,
    embedding: &[f32],
) -> Result<TopicRoot> {
    let q = query(
        "CREATE (t:TopicRoot {id: $id, name: $name, description: $description, embedding: $embedding, created_at: datetime()}) RETURN t.id, t.name, t.description"
    )
    .param("id", id.to_string())
    .param("name", name)
    .param("description", description)
    .param("embedding", embedding.to_vec());

    graph.run(q).await?;

    Ok(TopicRoot { id, name: name.to_string(), description: description.to_string() })
}

pub async fn find_topic_root_by_id(graph: &Graph, id: Uuid) -> Result<Option<TopicRoot>> {
    let mut result = graph
        .execute(
            query("MATCH (t:TopicRoot {id: $id}) RETURN t.id AS id, t.name AS name, t.description AS description")
                .param("id", id.to_string()),
        )
        .await?;

    if let Some(row) = result.next().await? {
        let id_str: String = row.get("id")?;
        Ok(Some(TopicRoot {
            id: id_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
            name: row.get("name")?,
            description: row.get("description")?,
        }))
    } else {
        Ok(None)
    }
}

// ─── Node operations ───────────────────────────────────────────────

pub async fn create_node(
    graph: &Graph,
    id: Uuid,
    title: &str,
    description: &str,
    topic_root_id: Uuid,
    resources: &[Resource],
    embedding: &[f32],
) -> Result<Node> {
    let resources_json = serde_json::to_string(resources)
        .map_err(|e| Error::Internal(e.to_string()))?;

    let q = query(
        "CREATE (n:Node {id: $id, title: $title, description: $description, topic_root_id: $topic_root_id, resources: $resources, embedding: $embedding, visit_count: 0, created_at: datetime()}) RETURN n.id"
    )
    .param("id", id.to_string())
    .param("title", title)
    .param("description", description)
    .param("topic_root_id", topic_root_id.to_string())
    .param("resources", resources_json)
    .param("embedding", embedding.to_vec());

    graph.run(q).await?;

    Ok(Node {
        id,
        title: title.to_string(),
        description: description.to_string(),
        topic_root_id,
        resources: resources.to_vec(),
        visit_count: 0,
    })
}

pub async fn get_node(graph: &Graph, id: Uuid) -> Result<Option<Node>> {
    let mut result = graph
        .execute(
            query("MATCH (n:Node {id: $id}) RETURN n.id AS id, n.title AS title, n.description AS description, n.topic_root_id AS topic_root_id, n.resources AS resources, n.visit_count AS visit_count")
                .param("id", id.to_string()),
        )
        .await?;

    if let Some(row) = result.next().await? {
        let id_str: String = row.get("id")?;
        let topic_root_id_str: String = row.get("topic_root_id")?;
        let resources_str: String = row.get("resources")?;

        Ok(Some(Node {
            id: id_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
            title: row.get("title")?,
            description: row.get("description")?,
            topic_root_id: topic_root_id_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
            resources: serde_json::from_str(&resources_str).unwrap_or_default(),
            visit_count: row.get("visit_count")?,
        }))
    } else {
        Ok(None)
    }
}

// ─── Edge operations ───────────────────────────────────────────────

pub async fn create_edge(
    graph: &Graph,
    source_id: Uuid,
    target_id: Uuid,
    movement: Movement,
) -> Result<()> {
    let rel_type = movement.as_relationship_type();

    // Use MERGE to avoid duplicates
    let cypher = format!(
        "MATCH (a {{id: $source_id}}), (b {{id: $target_id}}) MERGE (a)-[r:{rel_type}]->(b) ON CREATE SET r.weight = 0.0, r.traversals = 0, r.backups = 0"
    );

    graph
        .run(
            query(&cypher)
                .param("source_id", source_id.to_string())
                .param("target_id", target_id.to_string()),
        )
        .await?;

    Ok(())
}

pub async fn create_starts_with_edge(
    graph: &Graph,
    topic_root_id: Uuid,
    node_id: Uuid,
) -> Result<()> {
    graph
        .run(
            query("MATCH (t:TopicRoot {id: $topic_root_id}), (n:Node {id: $node_id}) MERGE (t)-[:STARTS_WITH]->(n)")
                .param("topic_root_id", topic_root_id.to_string())
                .param("node_id", node_id.to_string()),
        )
        .await?;

    Ok(())
}

pub async fn increment_traversal(
    graph: &Graph,
    source_id: Uuid,
    target_id: Uuid,
    movement: Movement,
) -> Result<()> {
    let rel_type = movement.as_relationship_type();
    let cypher = format!(
        "MATCH (a {{id: $source_id}})-[r:{rel_type}]->(b {{id: $target_id}}) SET r.traversals = r.traversals + 1, r.weight = CASE WHEN (r.traversals + 1 + r.backups) = 0 THEN 0.0 ELSE toFloat(CASE WHEN (r.traversals + 1 - 2 * r.backups) < 0 THEN 0 ELSE r.traversals + 1 - 2 * r.backups END) / toFloat(r.traversals + 1 + r.backups) END"
    );

    graph
        .run(
            query(&cypher)
                .param("source_id", source_id.to_string())
                .param("target_id", target_id.to_string()),
        )
        .await?;

    // Increment visit count on target node
    graph
        .run(
            query("MATCH (n {id: $id}) SET n.visit_count = n.visit_count + 1")
                .param("id", target_id.to_string()),
        )
        .await?;

    Ok(())
}

pub async fn increment_backup(
    graph: &Graph,
    source_id: Uuid,
    target_id: Uuid,
    movement: Movement,
) -> Result<()> {
    let rel_type = movement.as_relationship_type();
    let cypher = format!(
        "MATCH (a {{id: $source_id}})-[r:{rel_type}]->(b {{id: $target_id}}) SET r.backups = r.backups + 1, r.weight = CASE WHEN (r.traversals + r.backups + 1) = 0 THEN 0.0 ELSE toFloat(CASE WHEN (r.traversals - 2 * (r.backups + 1)) < 0 THEN 0 ELSE r.traversals - 2 * (r.backups + 1) END) / toFloat(r.traversals + r.backups + 1) END"
    );

    graph
        .run(
            query(&cypher)
                .param("source_id", source_id.to_string())
                .param("target_id", target_id.to_string()),
        )
        .await?;

    Ok(())
}

// ─── Visible subgraph query ────────────────────────────────────────

pub struct VisibleSubgraph {
    pub topic_root: TopicRoot,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub async fn fetch_visible_subgraph(
    graph: &Graph,
    topic_root_id: Uuid,
    visited_ids: &[Uuid],
    current_node_id: Uuid,
) -> Result<VisibleSubgraph> {
    // Get topic root
    let topic_root = find_topic_root_by_id(graph, topic_root_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("TopicRoot {}", topic_root_id)))?;

    let visited_strs: Vec<String> = visited_ids.iter().map(|id| id.to_string()).collect();
    let current_str = current_node_id.to_string();

    // Collect all node IDs we need (visited + current)
    let mut all_ids = visited_strs.clone();
    if !all_ids.contains(&current_str) {
        all_ids.push(current_str.clone());
    }

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Fetch all visited/current nodes
    for id_str in &all_ids {
        let mut result = graph.execute(
            query("MATCH (n:Node {id: $id}) RETURN n.id AS id, n.title AS title, n.description AS description, n.topic_root_id AS topic_root_id, n.resources AS resources, n.visit_count AS visit_count")
                .param("id", id_str.as_str())
        ).await?;

        if let Some(row) = result.next().await? {
            let node_id: String = row.get("id")?;
            let trid: String = row.get("topic_root_id")?;
            let resources_str: String = row.get("resources")?;

            nodes.push(Node {
                id: node_id.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                title: row.get("title")?,
                description: row.get("description")?,
                topic_root_id: trid.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                resources: serde_json::from_str(&resources_str).unwrap_or_default(),
                visit_count: row.get("visit_count")?,
            });
        }
    }

    // Fetch edges between visited nodes and from visited nodes to proposals
    for movement in Movement::all() {
        let rel_type = movement.as_relationship_type();

        // Edges from visited/current nodes to any node
        let cypher = format!(
            "MATCH (a)-[r:{rel_type}]->(b) WHERE a.id IN $ids RETURN a.id AS source_id, b.id AS target_id, r.weight AS weight, r.traversals AS traversals, r.backups AS backups, b.title AS b_title, b.description AS b_description, b.topic_root_id AS b_topic_root_id, b.resources AS b_resources, b.visit_count AS b_visit_count"
        );

        let mut result = graph.execute(
            query(&cypher).param("ids", all_ids.clone())
        ).await?;

        while let Some(row) = result.next().await? {
            let source_str: String = row.get("source_id")?;
            let target_str: String = row.get("target_id")?;

            edges.push(Edge {
                source_id: source_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                target_id: target_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                movement: *movement,
                weight: row.get("weight")?,
                traversals: row.get("traversals")?,
                backups: row.get("backups")?,
            });

            // If target node is not already in our list, add it (it's a proposal)
            let target_id: Uuid = target_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?;
            if !nodes.iter().any(|n| n.id == target_id) {
                let trid: String = row.get("b_topic_root_id")?;
                let resources_str: String = row.get("b_resources")?;

                nodes.push(Node {
                    id: target_id,
                    title: row.get("b_title")?,
                    description: row.get("b_description")?,
                    topic_root_id: trid.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                    resources: serde_json::from_str(&resources_str).unwrap_or_default(),
                    visit_count: row.get("b_visit_count")?,
                });
            }
        }
    }

    // Also fetch STARTS_WITH edges from topic root
    let mut starts_result = graph.execute(
        query("MATCH (t:TopicRoot {id: $id})-[:STARTS_WITH]->(n:Node) RETURN n.id AS id, n.title AS title, n.description AS description, n.topic_root_id AS topic_root_id, n.resources AS resources, n.visit_count AS visit_count")
            .param("id", topic_root_id.to_string())
    ).await?;

    while let Some(row) = starts_result.next().await? {
        let node_id_str: String = row.get("id")?;
        let node_id: Uuid = node_id_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?;

        if !nodes.iter().any(|n| n.id == node_id) {
            let trid: String = row.get("topic_root_id")?;
            let resources_str: String = row.get("resources")?;

            nodes.push(Node {
                id: node_id,
                title: row.get("title")?,
                description: row.get("description")?,
                topic_root_id: trid.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                resources: serde_json::from_str(&resources_str).unwrap_or_default(),
                visit_count: row.get("visit_count")?,
            });
        }

        // Add a SUPPORTS edge from root to starting point for rendering
        if !edges.iter().any(|e| e.source_id == topic_root_id && e.target_id == node_id) {
            edges.push(Edge {
                source_id: topic_root_id,
                target_id: node_id,
                movement: Movement::Supports,
                weight: 0.0,
                traversals: 0,
                backups: 0,
            });
        }
    }

    Ok(VisibleSubgraph { topic_root, nodes, edges })
}

// ─── Vector search ─────────────────────────────────────────────────

pub async fn vector_search_topic_roots(
    graph: &Graph,
    embedding: &[f32],
    limit: usize,
) -> Result<Vec<(TopicRoot, f64)>> {
    let mut result = graph.execute(
        query("CALL vector_search.search('topic_root_embedding', $limit, $embedding) YIELD node, similarity RETURN node.id AS id, node.name AS name, node.description AS description, similarity")
            .param("embedding", embedding.to_vec())
            .param("limit", limit as i64)
    ).await?;

    let mut results = Vec::new();
    while let Some(row) = result.next().await? {
        let id_str: String = row.get("id")?;
        let similarity: f64 = row.get("similarity")?;

        results.push((
            TopicRoot {
                id: id_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                name: row.get("name")?,
                description: row.get("description")?,
            },
            similarity,
        ));
    }

    Ok(results)
}

pub async fn vector_search_nodes(
    graph: &Graph,
    embedding: &[f32],
    topic_root_id: Uuid,
    limit: usize,
) -> Result<Vec<(Node, f64)>> {
    let mut result = graph.execute(
        query("CALL vector_search.search('node_embedding', $limit, $embedding) YIELD node, similarity WHERE node.topic_root_id = $topic_root_id RETURN node.id AS id, node.title AS title, node.description AS description, node.topic_root_id AS topic_root_id, node.resources AS resources, node.visit_count AS visit_count, similarity")
            .param("embedding", embedding.to_vec())
            .param("topic_root_id", topic_root_id.to_string())
            .param("limit", limit as i64)
    ).await?;

    let mut results = Vec::new();
    while let Some(row) = result.next().await? {
        let id_str: String = row.get("id")?;
        let trid: String = row.get("topic_root_id")?;
        let resources_str: String = row.get("resources")?;
        let similarity: f64 = row.get("similarity")?;

        results.push((
            Node {
                id: id_str.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                title: row.get("title")?,
                description: row.get("description")?,
                topic_root_id: trid.parse().map_err(|_| Error::Internal("invalid uuid".into()))?,
                resources: serde_json::from_str(&resources_str).unwrap_or_default(),
                visit_count: row.get("visit_count")?,
            },
            similarity,
        ));
    }

    Ok(results)
}
