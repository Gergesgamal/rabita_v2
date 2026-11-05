// backend/src/auth/session.rs

//! Session management with rotation and revocation per D-014.

use crate::{error::{AppError, AppResult}, state::AppState};
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionClaim {
    #[serde(default)]
    pub consumer_id: Option<String>,
    pub session_id: Uuid,
    pub account_id: Uuid,
    pub context_id: Uuid,
    pub role: String,
    pub permission_version: i64,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub struct SessionManager;

impl SessionManager {
    /// Issue new session.
    pub async fn issue_session(
        cache: &ConnectionManager,
        account_id: Uuid,
        context_id: Uuid,
        role: String,
        absolute_ttl_secs: u64,
    ) -> AppResult<(String, SessionClaim)> {
        let session_id = Uuid::new_v4();
        let token = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires = now + Duration::seconds(absolute_ttl_secs as i64);
        
        let claim = SessionClaim {
            session_id,
            account_id,
            context_id,
            consumer_id: None,
            role,
            permission_version: 1,
            issued_at: now,
            expires_at: expires,
        };
        
        let key = format!("session:{}", token);
        let mut conn = cache.clone();
        
        redis::cmd("SET")
            .arg(&key)
            .arg(serde_json::to_string(&claim)?)
            .arg("EX")
            .arg(absolute_ttl_secs)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        Ok((token, claim))
    }

    /// Verify and retrieve session.
    pub async fn verify_session(cache: &ConnectionManager, token: &str) -> AppResult<SessionClaim> {
        let key = format!("session:{}", token);
        let mut conn = cache.clone();
        
        let data: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await?;
        
        match data {
            Some(json) => {
                let claim: SessionClaim = serde_json::from_str(&json)?;
                
                // Check expiry
                if Utc::now() > claim.expires_at {
                    redis::cmd("DEL")
                        .arg(&key)
                        .query_async::<_, ()>(&mut conn)
                        .await?;
                    return Err(AppError::SessionExpired);
                }
                
                Ok(claim)
            }
            None => Err(AppError::SessionExpired),
        }
    }

    /// Rotate session token.
    pub async fn rotate_session(
        cache: &ConnectionManager,
        old_token: &str,
        absolute_ttl_secs: u64,
    ) -> AppResult<String> {
        let claim = Self::verify_session(cache, old_token).await?;
        let new_token = uuid::Uuid::new_v4().to_string();
        
        let old_key = format!("session:{}", old_token);
        let new_key = format!("session:{}", new_token);
        
        let mut conn = cache.clone();
        
        // Delete old
        redis::cmd("DEL")
            .arg(&old_key)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        // Create new with same claim
        redis::cmd("SET")
            .arg(&new_key)
            .arg(serde_json::to_string(&claim)?)
            .arg("EX")
            .arg(absolute_ttl_secs)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        Ok(new_token)
    }

    /// Revoke session.
    pub async fn revoke_session(cache: &ConnectionManager, token: &str) -> AppResult<()> {
        let key = format!("session:{}", token);
        let mut conn = cache.clone();
        
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        Ok(())
    }
}
