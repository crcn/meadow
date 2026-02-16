use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

use super::guard::get_member_id;
use super::types::{AiResponse, AuthResult, Movement, TopicGraph, VerifyResult};
use crate::state::AppState;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // ─── Auth ────────────────────────────────────────────────────────

    /// Send an OTP code to the given phone number.
    async fn send_otp(&self, ctx: &Context<'_>, phone: String) -> Result<AuthResult> {
        let state = ctx.data::<AppState>()?;
        state.otp.send(&phone).await?;
        Ok(AuthResult { success: true })
    }

    /// Verify the OTP code. Returns a JWT token and member ID.
    async fn verify_otp(
        &self,
        ctx: &Context<'_>,
        phone: String,
        code: String,
    ) -> Result<VerifyResult> {
        let state = ctx.data::<AppState>()?;

        state.otp.verify(&phone, &code).await?;

        let (member, identifier) = learning_core::domains::auth::identifier::find_or_create_member(
            &state.db,
            &phone,
            &state.admin_identifiers,
        )
        .await?;

        let token = learning_core::domains::auth::jwt::create_token(
            member.id,
            identifier.is_admin,
            &state.jwt_secret,
            &state.jwt_issuer,
        )?;

        Ok(VerifyResult {
            member_id: member.id.to_string().into(),
            token,
        })
    }

    // ─── Topic Entry ─────────────────────────────────────────────────

    /// Enter a topic by interest string. Matches or creates a TopicRoot,
    /// then generates starting points via AI investigation.
    async fn enter_topic(
        &self,
        ctx: &Context<'_>,
        interest: String,
    ) -> Result<TopicGraph> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;

        let embed_agent = create_embed_agent();

        let (topic_root, is_new) = learning_core::domains::graph::entry::match_or_create_topic(
            &state.graph,
            &embed_agent,
            &interest,
        )
        .await?;

        if is_new {
            let ai_agent = create_ai_agent();

            learning_core::domains::ai::starting_points::generate_starting_points(
                &ai_agent,
                &embed_agent,
                state.graph.clone(),
                &state.db,
                member_id,
                topic_root.id,
                &topic_root.name,
                &state.tavily_api_key,
                &state.youtube_api_key,
                state.ai_max_turns,
            )
            .await?;
        } else {
            // Existing topic — just set position to topic root if no position exists
            let existing_pos: Option<(Uuid,)> = sqlx::query_as(
                "SELECT current_node_id FROM learner_position WHERE member_id = $1 AND topic_root_id = $2"
            )
            .bind(member_id)
            .bind(topic_root.id)
            .fetch_optional(&state.db)
            .await?;

            if existing_pos.is_none() {
                sqlx::query(
                    "INSERT INTO learner_position (member_id, topic_root_id, current_node_id) VALUES ($1, $2, $3)"
                )
                .bind(member_id)
                .bind(topic_root.id)
                .bind(topic_root.id)
                .execute(&state.db)
                .await?;
            }
        }

        let graph = learning_core::domains::graph::assembler::assemble_topic_graph(
            &state.graph,
            &state.db,
            member_id,
            topic_root.id,
        )
        .await?;

        Ok(graph.into())
    }

    // ─── Traversal ───────────────────────────────────────────────────

    /// Traverse from current node to a target node via a movement.
    async fn traverse(
        &self,
        ctx: &Context<'_>,
        topic_root_id: ID,
        from_node_id: ID,
        to_node_id: ID,
        movement: Movement,
    ) -> Result<TopicGraph> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;

        let topic_root_id: Uuid = topic_root_id.parse()?;
        let from_node_id: Uuid = from_node_id.parse()?;
        let to_node_id: Uuid = to_node_id.parse()?;

        let graph = learning_core::domains::graph::traversal::traverse(
            &state.graph,
            &state.db,
            member_id,
            topic_root_id,
            from_node_id,
            to_node_id,
            movement.into(),
        )
        .await?;

        Ok(graph.into())
    }

    /// Back up to the previous node, downvoting the abandoned edge.
    async fn back_up(
        &self,
        ctx: &Context<'_>,
        topic_root_id: ID,
    ) -> Result<TopicGraph> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;
        let topic_root_id: Uuid = topic_root_id.parse()?;

        let graph = learning_core::domains::graph::traversal::back_up(
            &state.graph,
            &state.db,
            member_id,
            topic_root_id,
        )
        .await?;

        Ok(graph.into())
    }

    // ─── Proposals ───────────────────────────────────────────────────

    /// Generate more proposals from the current node via AI investigation.
    async fn show_more(
        &self,
        ctx: &Context<'_>,
        topic_root_id: ID,
        node_id: ID,
        node_title: String,
    ) -> Result<TopicGraph> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;
        let topic_root_id: Uuid = topic_root_id.parse()?;
        let node_id: Uuid = node_id.parse()?;

        let ai_agent = create_ai_agent();
        let embed_agent = create_embed_agent();

        let graph = learning_core::domains::ai::proposer::generate_proposals(
            &ai_agent,
            &embed_agent,
            state.graph.clone(),
            &state.db,
            member_id,
            topic_root_id,
            node_id,
            &node_title,
            &state.tavily_api_key,
            &state.youtube_api_key,
            state.ai_max_turns,
        )
        .await?;

        Ok(graph.into())
    }

    // ─── Notes ───────────────────────────────────────────────────────

    /// Leave a trail note on a node.
    async fn leave_note(
        &self,
        ctx: &Context<'_>,
        node_id: ID,
        body: String,
    ) -> Result<TopicGraph> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;
        let node_id: Uuid = node_id.parse()?;

        // Insert the note
        sqlx::query(
            "INSERT INTO node_notes (member_id, node_id, body) VALUES ($1, $2, $3)"
        )
        .bind(member_id)
        .bind(node_id)
        .bind(&body)
        .execute(&state.db)
        .await?;

        // Find topic_root_id for this node so we can return the graph
        let topic_root_id = find_topic_root_for_node(&state, node_id).await?;

        let graph = learning_core::domains::graph::assembler::assemble_topic_graph(
            &state.graph,
            &state.db,
            member_id,
            topic_root_id,
        )
        .await?;

        Ok(graph.into())
    }

    // ─── AI Ask ──────────────────────────────────────────────────────

    /// Ask the AI a question about a specific node.
    async fn ask_about_node(
        &self,
        ctx: &Context<'_>,
        topic_root_id: ID,
        node_id: ID,
        question: String,
    ) -> Result<AiResponse> {
        let member_id = get_member_id(ctx)?;
        let state = ctx.data::<AppState>()?;
        let topic_root_id: Uuid = topic_root_id.parse()?;
        let node_id: Uuid = node_id.parse()?;

        // Assemble the graph to get the full node context
        let graph = learning_core::domains::graph::assembler::assemble_topic_graph(
            &state.graph,
            &state.db,
            member_id,
            topic_root_id,
        )
        .await?;

        let node = graph
            .nodes
            .iter()
            .find(|n| n.id == node_id)
            .ok_or_else(|| async_graphql::Error::new("Node not found in graph"))?;

        // Build path from traversal history
        // Use visited node titles as the learning path context
        let visited_titles: Vec<String> = graph
            .nodes
            .iter()
            .filter(|n| {
                n.state == learning_core::domains::graph::models::NodeState::Visited
                    || n.state == learning_core::domains::graph::models::NodeState::Current
            })
            .map(|n| n.title.clone())
            .collect();

        let ai_agent = create_ai_agent();
        let answer = learning_core::domains::ai::ask::ask_about_node(
            &ai_agent,
            node,
            &visited_titles,
            &question,
        )
        .await?;

        Ok(AiResponse { answer })
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────

fn create_ai_agent() -> ai_client::OpenRouter {
    ai_client::OpenRouter::from_env("google/gemini-2.5-flash")
        .expect("OPENROUTER_API_KEY must be set")
        .with_app_name("ourmeadow")
}

fn create_embed_agent() -> ai_client::OpenRouter {
    ai_client::OpenRouter::from_env("openai/text-embedding-3-small")
        .expect("OPENROUTER_API_KEY must be set")
        .with_app_name("ourmeadow")
}

async fn find_topic_root_for_node(state: &AppState, node_id: Uuid) -> Result<Uuid> {
    // Look up the topic_root_id from learner_position where the member was on this node
    // or from the node's topic_root_id in Memgraph
    let mut result = state
        .graph
        .execute(
            neo4rs::query("MATCH (n {id: $id}) RETURN n.topic_root_id AS topic_root_id")
                .param("id", node_id.to_string()),
        )
        .await?;

    if let Some(row) = result.next().await? {
        let id_str: String = row.get("topic_root_id")?;
        let topic_root_id: Uuid = id_str
            .parse()
            .map_err(|_| async_graphql::Error::new("Invalid topic_root_id"))?;
        return Ok(topic_root_id);
    }

    Err(async_graphql::Error::new("Could not find topic for this node"))
}
