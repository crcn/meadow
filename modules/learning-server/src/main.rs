use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use neo4rs::Graph;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use learning_core::domains::auth::otp::OtpService;

mod graphql;
mod state;

use graphql::guard::extract_token_from_cookies;
use graphql::schema::{build_schema, AppSchema};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenvy::dotenv().ok();

    // Tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Postgres
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    tracing::info!("Connected to Postgres");

    // Run migrations
    sqlx::migrate!("../../migrations").run(&db).await?;
    tracing::info!("Migrations complete");

    // Memgraph
    let memgraph_uri = std::env::var("MEMGRAPH_URI").unwrap_or_else(|_| "bolt://localhost:7687".into());
    let memgraph_user = std::env::var("MEMGRAPH_USER").unwrap_or_else(|_| "".into());
    let memgraph_pass = std::env::var("MEMGRAPH_PASSWORD").unwrap_or_else(|_| "".into());

    let graph = Arc::new(
        Graph::new(&memgraph_uri, &memgraph_user, &memgraph_pass).await?,
    );
    tracing::info!("Connected to Memgraph");

    // Set up Memgraph schema (constraints, indexes)
    learning_core::domains::graph::client::setup_schema(&graph).await?;
    tracing::info!("Memgraph schema initialized");

    // OTP service
    let twilio_account_sid = std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_default();
    let twilio_auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default();
    let twilio_service_sid = std::env::var("TWILIO_VERIFY_SERVICE_SID").unwrap_or_default();
    let test_enabled = std::env::var("TEST_IDENTIFIER_ENABLED")
        .unwrap_or_else(|_| "false".into())
        .parse::<bool>()
        .unwrap_or(false);

    let otp = Arc::new(OtpService::new(
        twilio_account_sid,
        twilio_auth_token,
        twilio_service_sid,
        test_enabled,
    ));

    // App state
    let state = AppState {
        db,
        graph,
        otp,
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        jwt_issuer: std::env::var("JWT_ISSUER").unwrap_or_else(|_| "ourmeadow".into()),
        admin_identifiers: std::env::var("ADMIN_IDENTIFIERS").unwrap_or_default(),
        tavily_api_key: std::env::var("TAVILY_API_KEY").unwrap_or_default(),
        youtube_api_key: std::env::var("YOUTUBE_API_KEY").unwrap_or_default(),
        ai_max_turns: std::env::var("AI_MAX_TURNS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5),
    };

    // Build GraphQL schema
    let schema = build_schema();

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE]);

    // Router
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql))
        .route("/health", get(health))
        .layer(cors)
        .with_state(ServerState {
            schema,
            app_state: state,
        });

    let port = std::env::var("PORT").unwrap_or_else(|_| "4000".into());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ─── Axum state ──────────────────────────────────────────────────────

#[derive(Clone)]
struct ServerState {
    schema: AppSchema,
    app_state: AppState,
}

// ─── Handlers ────────────────────────────────────────────────────────

async fn graphql_handler(
    State(state): State<ServerState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let cookie_header = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok());

    let auth_token = extract_token_from_cookies(cookie_header);

    let request = req
        .into_inner()
        .data(state.app_state)
        .data(auth_token);

    state.schema.execute(request).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}
