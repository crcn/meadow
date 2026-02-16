use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

use super::guard::get_member_id;
use super::types::{SessionTopic, TopicGraph, TopicRoot};
use crate::state::AppState;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// List all topics the learner has started, ordered by last activity.
    async fn my_topics(&self, ctx: &Context<'_>) -> Result<Vec<SessionTopic>> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;

        let rows: Vec<SessionTopicRow> = sqlx::query_as(
            "SELECT lp.topic_root_id, lp.current_node_id, lp.updated_at FROM learner_position lp WHERE lp.member_id = $1 ORDER BY lp.updated_at DESC"
        )
        .bind(member_id)
        .fetch_all(&state.db)
        .await?;

        let mut topics = Vec::new();
        for row in rows {
            let topic_root = learning_core::domains::graph::queries::find_topic_root_by_id(
                &state.graph,
                row.topic_root_id,
            )
            .await?;

            if let Some(tr) = topic_root {
                topics.push(SessionTopic {
                    topic_root: TopicRoot::from(tr),
                    current_node_id: row.current_node_id.to_string().into(),
                    last_active: row.updated_at.to_rfc3339(),
                });
            }
        }

        Ok(topics)
    }

    /// Get the full graph for a specific topic.
    async fn topic_graph(
        &self,
        ctx: &Context<'_>,
        topic_root_id: ID,
    ) -> Result<TopicGraph> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;
        let topic_root_id: Uuid = topic_root_id.parse()?;

        let graph = learning_core::domains::graph::assembler::assemble_topic_graph(
            &state.graph,
            &state.db,
            member_id,
            topic_root_id,
        )
        .await?;

        Ok(graph.into())
    }
}

#[derive(sqlx::FromRow)]
struct SessionTopicRow {
    topic_root_id: Uuid,
    current_node_id: Uuid,
    updated_at: chrono::DateTime<chrono::Utc>,
}
