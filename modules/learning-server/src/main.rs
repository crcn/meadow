mod routes;

use anyhow::Result;
use clap::Parser;
use learning_core::{AppConfig, FileConfig, PromptRegistry, ServerDeps};
use learning_domains::curriculum::restate::{
    CurriculumWorkflowImpl, NodeObjectImpl, PrefetchServiceImpl,
};
use restate_sdk::prelude::*;
use sqlx::postgres::PgPoolOptions;

#[derive(Parser)]
#[command(name = "learning-server")]
struct Cli {
    /// Path to the config TOML file
    #[arg(long, default_value = "config/learning.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load config
    let config = AppConfig::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(&config.rust_log)
        .init();

    let file_config = FileConfig::load(&cli.config)?;
    let prompts = PromptRegistry::load(&file_config.prompts.dir)?;

    tracing::info!("Starting learning-server");

    // Database pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Database connected");

    // Run migrations
    sqlx::migrate!("../../migrations")
        .run(&db_pool)
        .await?;

    tracing::info!("Migrations applied");

    // Build shared deps
    let deps = ServerDeps::new(db_pool.clone(), config.clone(), file_config, prompts);

    // Build Axum router
    let axum_addr = config.server_addr();
    let app = routes::router(deps.clone());

    // Build Restate endpoint
    let restate_addr = config.restate_addr();
    let endpoint = Endpoint::builder()
        .bind(CurriculumWorkflowImpl::with_deps(deps.clone()).serve())
        .bind(NodeObjectImpl::with_deps(deps.clone()).serve())
        .bind(PrefetchServiceImpl::with_deps(deps.clone()).serve())
        .build();

    // Start both servers concurrently
    let axum_handle = tokio::spawn({
        let addr = axum_addr.clone();
        async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            tracing::info!("Axum server listening on {}", addr);
            axum::serve(listener, app).await?;
            Ok::<(), anyhow::Error>(())
        }
    });

    let restate_handle = tokio::spawn({
        let addr = restate_addr.clone();
        async move {
            tracing::info!("Restate endpoint listening on {}", addr);
            HttpServer::new(endpoint)
                .listen_and_serve(addr.parse().unwrap())
                .await;
            Ok::<(), anyhow::Error>(())
        }
    });

    // Auto-register with Restate admin
    let register_handle = tokio::spawn({
        let admin_url = config.restate_admin_url.clone();
        let restate_addr = restate_addr.clone();
        async move {
            // Wait for Restate endpoint to be ready
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            let client = reqwest::Client::new();
            let deployment_url = format!("http://{}:{}", "learning-server", restate_addr.split(':').last().unwrap_or("9080"));

            // In local dev without docker, use localhost
            let deployment_url = if std::env::var("DOCKER_ENV").is_ok() {
                deployment_url
            } else {
                format!("http://localhost:{}", restate_addr.split(':').last().unwrap_or("9080"))
            };

            tracing::info!("Registering deployment at {} with Restate admin {}", deployment_url, admin_url);

            match client
                .post(format!("{}/deployments", admin_url))
                .json(&serde_json::json!({ "uri": deployment_url }))
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    tracing::info!("Successfully registered with Restate");
                }
                Ok(resp) => {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    tracing::warn!("Restate registration returned {}: {}", status, body);
                }
                Err(e) => {
                    tracing::warn!("Failed to register with Restate (will retry on next request): {}", e);
                }
            }
        }
    });

    tokio::select! {
        r = axum_handle => { r??; }
        r = restate_handle => { r??; }
        _ = register_handle => {}
    }

    Ok(())
}
