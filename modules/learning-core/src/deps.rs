use crate::{AppConfig, FileConfig, PromptRegistry};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerDeps {
    pub db_pool: PgPool,
    pub http_client: reqwest::Client,
    pub config: Arc<AppConfig>,
    pub file_config: Arc<FileConfig>,
    pub prompts: Arc<PromptRegistry>,
}

impl ServerDeps {
    pub fn new(
        db_pool: PgPool,
        config: AppConfig,
        file_config: FileConfig,
        prompts: PromptRegistry,
    ) -> Self {
        Self {
            db_pool,
            http_client: reqwest::Client::new(),
            config: Arc::new(config),
            file_config: Arc::new(file_config),
            prompts: Arc::new(prompts),
        }
    }
}
