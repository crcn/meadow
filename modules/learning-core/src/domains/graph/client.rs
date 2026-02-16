use neo4rs::Graph;
use tracing::info;

use crate::error::Result;

pub async fn connect(uri: &str, user: &str, password: &str) -> Result<Graph> {
    let graph = Graph::new(uri, user, password).await?;
    info!("Connected to Memgraph at {}", uri);
    Ok(graph)
}

pub async fn setup_schema(graph: &Graph) -> Result<()> {
    // Constraints
    let constraints = [
        "CREATE CONSTRAINT ON (n:Node) ASSERT n.id IS UNIQUE",
        "CREATE CONSTRAINT ON (t:TopicRoot) ASSERT t.id IS UNIQUE",
    ];

    for query in &constraints {
        // Memgraph returns error if constraint already exists — that's fine
        if let Err(e) = graph.run(neo4rs::query(query)).await {
            let msg = e.to_string();
            if !msg.contains("already exists") {
                tracing::warn!("Constraint query warning: {}", msg);
            }
        }
    }

    // Indexes
    let indexes = [
        "CREATE INDEX ON :Node(topic_root_id)",
    ];

    for query in &indexes {
        if let Err(e) = graph.run(neo4rs::query(query)).await {
            let msg = e.to_string();
            if !msg.contains("already exists") {
                tracing::warn!("Index query warning: {}", msg);
            }
        }
    }

    // Vector indexes — Memgraph 3.8+ syntax
    let vector_indexes = [
        r#"CREATE VECTOR INDEX ON :Node(embedding) WITH CONFIG {"dimension": 1536, "metric": "cos"}"#,
        r#"CREATE VECTOR INDEX ON :TopicRoot(embedding) WITH CONFIG {"dimension": 1536, "metric": "cos"}"#,
    ];

    for query in &vector_indexes {
        if let Err(e) = graph.run(neo4rs::query(query)).await {
            let msg = e.to_string();
            if !msg.contains("already exists") {
                tracing::warn!("Vector index query warning: {}", msg);
            }
        }
    }

    info!("Memgraph schema setup complete");
    Ok(())
}
