// backend/src/state.rs

//! Application state shared across request handlers.

use crate::config::AppConfig;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL connection pool.
    pub db: PgPool,
    /// Valkey connection manager.
    pub cache: ConnectionManager,
    /// Application configuration.
    pub config: Arc<AppConfig>,
}

impl AppState {
    /// Create a new AppState.
    pub fn new(db: PgPool, cache: ConnectionManager, config: AppConfig) -> Self {
        Self {
            db,
            cache,
            config: Arc::new(config),
        }
    }
}
