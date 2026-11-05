// backend/src/auth/attempt_token.rs

//! Login attempt state machine in Valkey per D-015, D-023.

use crate::error::{AppError, AppResult};
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttemptState {
    Issued,
    PasswordOk,
    EnrollmentPending,
    MfaOk,
    SessionIssued,
    Expired,
    Burned,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptToken {
    pub token: String,
    pub state: AttemptState,
    pub account_id: Option<Uuid>,
    pub created_at: i64,
    pub ip_address: Option<String>,
}

impl AttemptToken {
    /// Create new attempt token (ISSUED state) per D-015.
    pub async fn issue(
        cache: &ConnectionManager,
        ttl_secs: u64,
        ip_address: Option<String>,
    ) -> AppResult<String> {
        let token = uuid::Uuid::new_v4().to_string();
        let key = format!("attempt_token:{}", token);
        
        let attempt = AttemptToken {
            token: token.clone(),
            state: AttemptState::Issued,
            account_id: None,
            created_at: chrono::Utc::now().timestamp(),
            ip_address,
        };
        
        let mut conn = cache.clone();
        redis::cmd("SET")
            .arg(&key)
            .arg(serde_json::to_string(&attempt)?)
            .arg("EX")
            .arg(ttl_secs)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        Ok(token)
    }

    /// Transition to PASSWORD_OK state and mint new token per D-015.
    pub async fn transition_to_password_ok(
        cache: &ConnectionManager,
        token: &str,
        account_id: Uuid,
        ttl_secs: u64,
    ) -> AppResult<String> {
        let key = format!("attempt_token:{}", token);
        let mut attempt: AttemptToken = Self::get(cache, token).await?;
        
        // Burn old token
        let mut conn = cache.clone();
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        // Mint new token
        let new_token = uuid::Uuid::new_v4().to_string();
        let new_key = format!("attempt_token:{}", new_token);
        
        attempt.token = new_token.clone();
        attempt.state = AttemptState::PasswordOk;
        attempt.account_id = Some(account_id);
        attempt.created_at = chrono::Utc::now().timestamp();
        
        redis::cmd("SET")
            .arg(&new_key)
            .arg(serde_json::to_string(&attempt)?)
            .arg("EX")
            .arg(ttl_secs)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        Ok(new_token)
    }

    /// Get attempt state from Valkey.
    pub async fn get(cache: &ConnectionManager, token: &str) -> AppResult<Self> {
        let key = format!("attempt_token:{}", token);
        let mut conn = cache.clone();
        
        let data: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await?;
        
        match data {
            Some(json) => serde_json::from_str(&json)
                .map_err(|_| AppError::InvalidToken),
            None => Err(AppError::InvalidToken),
        }
    }

    /// Burn token (terminal state produces non-authorizing audit record).
    pub async fn burn(cache: &ConnectionManager, token: &str) -> AppResult<()> {
        let key = format!("attempt_token:{}", token);
        let mut conn = cache.clone();
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<_, ()>(&mut conn)
            .await?;
        Ok(())
    }
}
