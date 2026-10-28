// backend/src/db.rs

//! Database connection pool setup and health checking.

use crate::config::DatabaseConfig;
use anyhow::Result;
use sqlx::postgres::PgPool;
use std::time::Duration;
use tracing::info;

/// Create a PostgreSQL connection pool with retry logic.
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let mut retries = 0;
    const MAX_RETRIES: u32 = 5;
    const RETRY_DELAY_MS: u64 = 500;

    loop {
        match try_create_pool(config).await {
            Ok(pool) => {
                info!(
                    "PostgreSQL pool created: {} max connections",
                    config.max_connections
                );
                return Ok(pool);
            }
            Err(e) if retries < MAX_RETRIES => {
                retries += 1;
                tracing::warn!(
                    "Failed to connect to PostgreSQL (attempt {}/{}): {}. Retrying in {}ms...",
                    retries,
                    MAX_RETRIES,
                    e,
                    RETRY_DELAY_MS
                );
                tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to create PostgreSQL pool after {} retries: {}",
                    MAX_RETRIES,
                    e
                ));
            }
        }
    }
}

async fn try_create_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let pool = PgPool::connect_lazy_with(
        sqlx::postgres::PgConnectOptions::new()
            .server_settings(std::collections::HashMap::from([
                ("application_name".to_string(), "rabita-backend".to_string()),
            ]))
            .from_url(config.database_url.parse()?)?,
    );

    // Test the connection
    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await?;

    Ok(pool)
}

/// Check if PostgreSQL is reachable.
pub async fn check_postgres(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("PostgreSQL health check failed: {}", e))
}

/// Check if Valkey is reachable.
pub async fn check_valkey(client: &redis::aio::ConnectionManager) -> Result<()> {
    let mut conn = client.clone();
    redis::cmd("PING")
        .query_async::<_, String>(&mut conn)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("Valkey health check failed: {}", e))
}
