// backend/src/idempotency/store.rs

//! Idempotency key storage per D-130: principal-scoped to prevent cross-principal replay.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use sha2::{Sha256, Digest};

pub struct IdempotencyStore;

impl IdempotencyStore {
    /// Store idempotency key response per D-130.
    /// 24-hour TTL, keyed by (principal_type, principal_id, key).
    pub async fn store(
        db: &PgPool,
        principal_type: &str,
        principal_id: &str,
        key: &str,
        method: &str,
        path: &str,
        payload: &[u8],
        status: u16,
        response_body: &serde_json::Value,
    ) -> AppResult<()> {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let payload_hash = format!("{:x}", hasher.finalize());

        let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);

        sqlx::query(
            r#"
            INSERT INTO app.idempotency_key
            (principal_type, principal_id, key, method, path, payload_hash, response_status, response_body, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (principal_type, principal_id, key) DO UPDATE
            SET response_status = $7, response_body = $8
            "#,
        )
        .bind(principal_type)
        .bind(principal_id)
        .bind(key)
        .bind(method)
        .bind(path)
        .bind(&payload_hash)
        .bind(status as i16)
        .bind(response_body)
        .bind(expires_at)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Retrieve idempotency key response per D-130.
    /// Returns (status, body) if found and payload hash matches.
    /// Returns 409 Conflict if key exists but payload_hash differs (mutated payload).
    pub async fn retrieve(
        db: &PgPool,
        principal_type: &str,
        principal_id: &str,
        key: &str,
        payload: &[u8],
    ) -> AppResult<Option<(u16, serde_json::Value)>> {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let payload_hash = format!("{:x}", hasher.finalize());

        let row: Option<(i16, String, serde_json::Value)> = sqlx::query_as(
            r#"
            SELECT response_status, payload_hash, response_body
            FROM app.idempotency_key
            WHERE principal_type = $1 AND principal_id = $2 AND key = $3 AND expires_at > now()
            "#,
        )
        .bind(principal_type)
        .bind(principal_id)
        .bind(key)
        .fetch_optional(db)
        .await?;

        match row {
            Some((status, stored_hash, body)) => {
                if stored_hash != payload_hash {
                    // Payload mutated
                    return Err(AppError::Conflict(
                        "Idempotency key payload mismatch".to_string(),
                    ));
                }
                Ok(Some((status as u16, body)))
            }
            None => Ok(None),
        }
    }

    /// Check if key requires idempotency per D-130.
    pub fn requires_idempotency(method: &str, path: &str) -> bool {
        // All mutations require idempotency key per D-130
        matches!(method, "POST" | "PUT" | "PATCH" | "DELETE")
    }
}
