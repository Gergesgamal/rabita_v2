// backend/src/delegation/execution_token.rs

//! Execution tokens bound to approval request and payload hash per D-026.

use crate::error::{AppError, AppResult};
use sha2::{Sha256, Digest};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ExecutionTokenManager;

impl ExecutionTokenManager {
    /// Create execution token bound to approval request and payload hash per D-026.
    /// Token is single-use with 10-minute TTL.
    pub async fn create(
        db: &PgPool,
        approval_request_id: Uuid,
        account_id: Uuid,
        employee_account_id: Option<Uuid>,
        payload_hash: &str,
    ) -> AppResult<String> {
        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        sqlx::query(
            r#"
            INSERT INTO app.execution_token
            (approval_request_id, account_id, employee_account_id, token_hash, payload_hash, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(approval_request_id)
        .bind(account_id)
        .bind(employee_account_id)
        .bind(&token) // In production, hash this
        .bind(payload_hash)
        .bind(expires_at)
        .execute(db)
        .await?;

        Ok(token)
    }

    /// Verify execution token: check hash matches payload and token is unused per D-026.
    pub async fn verify_and_consume(
        db: &PgPool,
        token: &str,
        payload: &serde_json::Value,
    ) -> AppResult<ExecutionTokenRow> {
        // Compute payload hash
        let payload_json = serde_json::to_string(payload)?;
        let mut hasher = Sha256::new();
        hasher.update(payload_json.as_bytes());
        let payload_hash = format!("{:x}", hasher.finalize());

        // Fetch token
        let token_row: ExecutionTokenRow = sqlx::query_as(
            r#"
            SELECT id, approval_request_id, account_id, employee_account_id, token_hash,
                   payload_hash, executed, expires_at
            FROM app.execution_token
            WHERE token_hash = $1 AND executed = false AND expires_at > now()
            "#,
        )
        .bind(token)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| {
            AppError::InvalidToken
        })?;

        // Verify payload hash matches
        if token_row.payload_hash != payload_hash {
            return Err(AppError::Conflict(
                "Payload does not match approval request".to_string(),
            ));
        }

        // Mark as executed
        sqlx::query(
            r#"
            UPDATE app.execution_token
            SET executed = true, executed_at = now()
            WHERE id = $1
            "#,
        )
        .bind(token_row.id)
        .execute(db)
        .await?;

        Ok(token_row)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ExecutionTokenRow {
    pub id: Uuid,
    pub approval_request_id: Uuid,
    pub account_id: Uuid,
    pub employee_account_id: Option<Uuid>,
    pub token_hash: String,
    pub payload_hash: String,
    pub executed: bool,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
