use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub restate_admin_url: String,
    pub restate_ingress_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub restate_port: u16,
    pub ai_provider: String,
    pub anthropic_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub youtube_api_key: Option<String>,
    pub rust_log: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            restate_admin_url: std::env::var("RESTATE_ADMIN_URL")
                .unwrap_or_else(|_| "http://localhost:9070".into()),
            restate_ingress_url: std::env::var("RESTATE_INGRESS_URL")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".into()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .context("SERVER_PORT must be a valid port number")?,
            restate_port: std::env::var("RESTATE_PORT")
                .unwrap_or_else(|_| "9080".into())
                .parse()
                .context("RESTATE_PORT must be a valid port number")?,
            ai_provider: std::env::var("AI_PROVIDER")
                .unwrap_or_else(|_| "anthropic".into()),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            youtube_api_key: std::env::var("YOUTUBE_API_KEY").ok(),
            rust_log: std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".into()),
        })
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn restate_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.restate_port)
    }
}
