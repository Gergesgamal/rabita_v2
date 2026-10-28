// backend/src/main.rs

//! Rabita backend entry point with graceful shutdown.

use anyhow::Result;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use backend::{
    config::AppConfig,
    db,
    routes::health,
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = AppConfig::from_env()?;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(&config.app.log_level)),
        )
        .json()
        .init();

    info!("Starting Rabita backend in {} environment", config.app.env);

    // Create database pool with retry
    let db_pool = db::create_pool(&config.database).await?;

    // Create Valkey connection manager
    let redis_client = redis::Client::open(config.cache.redis_url.as_str())?;
    let cache = redis::aio::ConnectionManager::new(redis_client).await?;

    info!("Database and cache connections established");

    // Create shared state
    let state = AppState::new(db_pool, cache, config.clone());

    // Build router
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::gzip());

    // Bind to socket
    let addr = config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Server listening on {}", addr);

    // Create server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shut down gracefully");

    Ok(())
}

/// Signal handler for graceful shutdown (SIGTERM/SIGINT).
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received CTRL+C signal");
        }
        _ = terminate => {
            info!("Received SIGTERM signal");
        }
    }
}
