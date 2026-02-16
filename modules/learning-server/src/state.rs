use neo4rs::Graph;
use sqlx::PgPool;
use std::sync::Arc;

use learning_core::domains::auth::otp::OtpService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub graph: Arc<Graph>,
    pub otp: Arc<OtpService>,
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub admin_identifiers: String,
    pub tavily_api_key: String,
    pub youtube_api_key: String,
    pub ai_max_turns: usize,
}
