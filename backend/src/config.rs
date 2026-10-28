// backend/src/config.rs

//! Configuration loading from environment variables and .env file.

use anyhow::Result;
use config::Config;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub auth: AuthConfig,
    pub aws: AwsConfig,
    pub app: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout_secs: u64,
    pub idle_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub redis_url: String,
    pub default_ttl_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub session_ttl_secs: u64,
    pub attempt_token_ttl_secs: u64,
    pub webauthn_rp_id: String,
    pub webauthn_rp_origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub s3_staging_bucket: String,
    pub s3_media_bucket: String,
    pub s3_quarantine_bucket: String,
    pub ses_from_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub env: String,
    pub log_level: String,
}

impl AppConfig {
    /// Load configuration from environment variables and .env file.
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let config = Config::builder()
            .add_source(config::Environment::default().try_parsing(true).separator("_"))
            .build()?;

        let cfg = config.try_deserialize::<AppConfig>()?;
        Ok(cfg)
    }

    /// Get the server socket address.
    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address")
    }
}
