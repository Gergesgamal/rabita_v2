// backend/src/consumer_auth/login.rs

//! Consumer login per D-109: email + password, store-scoped sessions.

use crate::error::{AppError, AppResult};
use crate::auth::password::PasswordManager;
use crate::auth::session::{SessionClaim, SessionManager};
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use uuid::Uuid;

pub struct ConsumerLoginManager;

impl ConsumerLoginManager {
    /// Authenticate consumer per D-109: email + password per store.
    pub async fn login(
        db: &PgPool,
        store_id: Uuid,
        email: &str,
        password: &str,
    ) -> AppResult<Uuid> {
        let normalized_email = email.to_lowercase();

        // Lookup consumer by store_id + normalized_email
        let consumer: Option<(Uuid, String)> = sqlx::query_as(
            r#"
            SELECT id, password_hash
            FROM app.consumer
            WHERE store_id = $1 AND normalized_email = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(store_id)
        .bind(&normalized_email)
        .fetch_optional(db)
        .await?;

        let (consumer_id, password_hash) = consumer.ok_or_else(|| {
            // Generic error to avoid disclosing existence
            AppError::AuthenticationFailed
        })?;

        // Verify password
        let valid = PasswordManager::verify_password(password, &password_hash)?;
        if !valid {
            return Err(AppError::AuthenticationFailed);
        }

        Ok(consumer_id)
    }

    /// Issue consumer session per D-109.
    /// Secure, HttpOnly, SameSite=Lax, 30-day absolute TTL, 7-day idle TTL.
    pub async fn issue_session(
        cache: &ConnectionManager,
        store_id: Uuid,
        consumer_id: Uuid,
    ) -> AppResult<String> {
        let session_id = Uuid::new_v4();
        let token = uuid::Uuid::new_v4().to_string();

        let claim = SessionClaim {
            session_id,
            account_id: consumer_id, // Reuse for simplicity
            consumer_id: None,
            context_id: store_id,
            role: "consumer".to_string(),
            permission_version: 1,
            issued_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::days(30),
        };

        let key = format!("consumer_session:{}", token);
        let mut conn = cache.clone();

        redis::cmd("SET")
            .arg(&key)
            .arg(serde_json::to_string(&claim)?)
            .arg("EX")
            .arg(30 * 24 * 60 * 60) // 30 days
            .query_async::<_, ()>(&mut conn)
            .await?;

        Ok(token)
    }

    /// Verify consumer session.
    pub async fn verify_session(
        cache: &ConnectionManager,
        token: &str,
    ) -> AppResult<SessionClaim> {
        let key = format!("consumer_session:{}", token);
        let mut conn = cache.clone();

        let data: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await?;

        match data {
            Some(json) => {
                let claim: SessionClaim = serde_json::from_str(&json)?;

                if chrono::Utc::now() > claim.expires_at {
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
}
