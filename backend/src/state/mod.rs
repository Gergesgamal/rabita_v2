use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::future::{ready, Ready};

/// Shared application state. Cloned per request by Axum.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cache: ConnectionManager,
}

/// Compatibility shim. Call sites written against the redis `Client` API get a cheap clone
/// of the shared, already-connected manager. Remove once call sites use `.cache.clone()`.
pub trait CacheConnection {
    fn get_connection(&self) -> Ready<redis::RedisResult<ConnectionManager>>;
}

impl CacheConnection for ConnectionManager {
    fn get_connection(&self) -> Ready<redis::RedisResult<ConnectionManager>> {
        ready(Ok(self.clone()))
    }
}
