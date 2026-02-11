use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use reqwest::Client;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod services;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub http_client: Client,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aidetector_server=debug,tower_http=debug".into()),
        )
        .init();

    let config = Config::from_env();
    let pool = db::init_pool(&config.database_url).await;
    let http_client = Client::new();

    let state = AppState {
        db: pool,
        http_client,
        config: config.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Protected routes (require API key)
    let protected = Router::new()
        .route("/api/analyze", post(routes::analyze::analyze))
        .route("/api/history", get(routes::history::history))
        .layer(middleware::from_fn(auth::require_api_key));

    let app = Router::new()
        .route("/api/health", get(routes::health::health))
        .merge(protected)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(axum::Extension(config.clone()))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Server starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
