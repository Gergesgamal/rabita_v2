// backend/src/delegation/approval_request.rs

//! Approval request creation and decision per D-026.

use crate::error::{AppError, AppResult};
use sha2::{Sha256, Digest};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ApprovalRequestManager;

impl ApprovalRequestManager {
    /// Create approval request for dual-control action per D-026.
    /// Payload is hashed for mutation detection.
    pub async fn create_request(
        db: &PgPool,
        account_id: Uuid,
        context_id: Uuid,
        action_type: &str,
        action_payload: &serde_json::Value,
    ) -> AppResult<Uuid> {
        // Compute SHA256 hash of payload
        let payload_json = serde_json::to_string(action_payload)?;
        let mut hasher = Sha256::new();
        hasher.update(payload_json.as_bytes());
        let payload_hash = format!("{:x}", hasher.finalize());

        // Calculate expiry (10 minutes per D-026)
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        let request_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.approval_request
            (account_id, context_id, action_type, action_payload, payload_hash, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
        )
        .bind(account_id)
        .bind(context_id)
        .bind(action_type)
        .bind(action_payload)
        .bind(&payload_hash)
        .bind(expires_at)
        .fetch_one(db)
        .await?;

        Ok(request_id)
    }

    /// Approve an approval request per D-026.
    pub async fn approve(
        db: &PgPool,
        request_id: Uuid,
        approved_by_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE app.approval_request
            SET state = 'approved', approved_by_id = $1, decided_at = now()
            WHERE id = $2 AND state = 'pending'
            "#,
        )
        .bind(approved_by_id)
        .bind(request_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Reject an approval request.
    pub async fn reject(
        db: &PgPool,
        request_id: Uuid,
        rejected_by_id: Uuid,
        reason: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE app.approval_request
            SET state = 'rejected', approved_by_id = $1, rejected_reason = $2, decided_at = now()
            WHERE id = $3 AND state = 'pending'
            "#,
        )
        .bind(rejected_by_id)
        .bind(reason)
        .bind(request_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Get approval request and check state.
    pub async fn get(
        db: &PgPool,
        request_id: Uuid,
    ) -> AppResult<ApprovalRequestRow> {
        sqlx::query_as::<_, ApprovalRequestRow>(
            r#"
            SELECT id, account_id, context_id, action_type, action_payload, payload_hash, state,
                   approved_by_id, decided_at, expires_at, created_at
            FROM app.approval_request
            WHERE id = $1
            "#,
        )
        .bind(request_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Approval request not found".to_string()))
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ApprovalRequestRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub context_id: Uuid,
    pub action_type: String,
    pub action_payload: serde_json::Value,
    pub payload_hash: String,
    pub state: String,
    pub approved_by_id: Option<Uuid>,
    pub decided_at: Option<chrono::DateTime<chrono::Utc>>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
