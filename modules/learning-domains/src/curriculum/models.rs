use learning_core::types::*;
use sqlx::PgPool;
use uuid::Uuid;

// ── Curriculum ──────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CurriculumRow {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub topic: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CurriculumRow {
    pub async fn insert(
        pool: &PgPool,
        title: &str,
        description: &str,
        topic: &str,
    ) -> anyhow::Result<Self> {
        let row = sqlx::query_as::<_, Self>(
            "INSERT INTO curricula (title, description, topic) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(title)
        .bind(description)
        .bind(topic)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
        let row = sqlx::query_as::<_, Self>("SELECT * FROM curricula WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(row)
    }

    pub async fn list_all(pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let rows =
            sqlx::query_as::<_, Self>("SELECT * FROM curricula ORDER BY created_at DESC")
                .fetch_all(pool)
                .await?;
        Ok(rows)
    }

    pub fn to_summary(&self) -> CurriculumSummary {
        CurriculumSummary {
            id: self.id,
            title: self.title.clone(),
            topic: self.topic.clone(),
            created_at: self.created_at,
        }
    }
}

// ── Node ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NodeRow {
    pub id: Uuid,
    pub curriculum_id: Uuid,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub node_type: NodeType,
    pub depth: i32,
    pub position: i32,
    pub status: NodeStatus,
    pub parent_node_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl NodeRow {
    pub async fn insert(
        pool: &PgPool,
        curriculum_id: Uuid,
        title: &str,
        summary: &str,
        content: &str,
        node_type: NodeType,
        depth: i32,
        position: i32,
        parent_node_id: Option<Uuid>,
    ) -> anyhow::Result<Self> {
        let row = sqlx::query_as::<_, Self>(
            "INSERT INTO nodes (curriculum_id, title, summary, content, node_type, depth, position, parent_node_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(curriculum_id)
        .bind(title)
        .bind(summary)
        .bind(content)
        .bind(node_type)
        .bind(depth)
        .bind(position)
        .bind(parent_node_id)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
        let row = sqlx::query_as::<_, Self>("SELECT * FROM nodes WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(row)
    }

    pub async fn find_children(pool: &PgPool, parent_id: Uuid) -> anyhow::Result<Vec<Self>> {
        let rows = sqlx::query_as::<_, Self>(
            "SELECT * FROM nodes WHERE parent_node_id = $1 ORDER BY position",
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_curriculum(
        pool: &PgPool,
        curriculum_id: Uuid,
        depth: Option<i32>,
    ) -> anyhow::Result<Vec<Self>> {
        let rows = if let Some(d) = depth {
            sqlx::query_as::<_, Self>(
                "SELECT * FROM nodes WHERE curriculum_id = $1 AND depth = $2 ORDER BY position",
            )
            .bind(curriculum_id)
            .bind(d)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, Self>(
                "SELECT * FROM nodes WHERE curriculum_id = $1 ORDER BY depth, position",
            )
            .bind(curriculum_id)
            .fetch_all(pool)
            .await?
        };
        Ok(rows)
    }

    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        status: NodeStatus,
    ) -> anyhow::Result<()> {
        sqlx::query("UPDATE nodes SET status = $1 WHERE id = $2")
            .bind(status)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub fn to_summary(&self) -> NodeSummary {
        NodeSummary {
            id: self.id,
            title: self.title.clone(),
            summary: self.summary.clone(),
            node_type: self.node_type,
            status: self.status,
            depth: self.depth,
            position: self.position,
        }
    }
}

// ── Edge ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EdgeRow {
    pub id: Uuid,
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub edge_type: EdgeType,
    pub position: i32,
}

impl EdgeRow {
    pub async fn insert(
        pool: &PgPool,
        from_node: Uuid,
        to_node: Uuid,
        edge_type: EdgeType,
        position: i32,
    ) -> anyhow::Result<Self> {
        let row = sqlx::query_as::<_, Self>(
            "INSERT INTO edges (from_node, to_node, edge_type, position)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (from_node, to_node, edge_type) DO NOTHING
             RETURNING *",
        )
        .bind(from_node)
        .bind(to_node)
        .bind(edge_type)
        .bind(position)
        .fetch_optional(pool)
        .await?
        .unwrap_or(Self {
            id: Uuid::new_v4(),
            from_node,
            to_node,
            edge_type,
            position,
        });
        Ok(row)
    }

    pub async fn find_from_node(pool: &PgPool, from_node: Uuid) -> anyhow::Result<Vec<Self>> {
        let rows = sqlx::query_as::<_, Self>(
            "SELECT * FROM edges WHERE from_node = $1 ORDER BY edge_type, position",
        )
        .bind(from_node)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_from_node_by_type(
        pool: &PgPool,
        from_node: Uuid,
        edge_type: EdgeType,
    ) -> anyhow::Result<Vec<Self>> {
        let rows = sqlx::query_as::<_, Self>(
            "SELECT * FROM edges WHERE from_node = $1 AND edge_type = $2 ORDER BY position",
        )
        .bind(from_node)
        .bind(edge_type)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }
}

// ── Video ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VideoRow {
    pub id: Uuid,
    pub node_id: Uuid,
    pub youtube_id: String,
    pub title: String,
    pub channel_name: String,
    pub thumbnail_url: String,
    pub duration_secs: i32,
    pub view_count: i64,
    pub rank: i32,
    pub relevance_score: f32,
    pub ai_rationale: String,
}

impl VideoRow {
    pub async fn find_by_node(pool: &PgPool, node_id: Uuid) -> anyhow::Result<Vec<Self>> {
        let rows = sqlx::query_as::<_, Self>(
            "SELECT * FROM videos WHERE node_id = $1 ORDER BY rank",
        )
        .bind(node_id)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub fn to_summary(&self) -> VideoSummary {
        VideoSummary {
            id: self.id,
            youtube_id: self.youtube_id.clone(),
            title: self.title.clone(),
            channel_name: self.channel_name.clone(),
            thumbnail_url: self.thumbnail_url.clone(),
            duration_secs: self.duration_secs,
            rank: self.rank,
            relevance_score: self.relevance_score,
            ai_rationale: self.ai_rationale.clone(),
        }
    }
}
