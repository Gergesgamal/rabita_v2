//! Audit log writer. Pass the transaction executor when the audited change is
//! part of a transaction, so the audit row commits or rolls back with it.

use crate::error::AppResult;
use sqlx::PgExecutor;
use uuid::Uuid;

pub struct AuditLogger;

impl AuditLogger {
    pub async fn log_action<'e, E>(
        executor: E,
        actor_account_id: Uuid,
        action: &str,
        target_id: Option<Uuid>,
        payload: serde_json::Value,
        reason: Option<&str>,
    ) -> AppResult<()>
    where
        E: PgExecutor<'e>,
    {
        sqlx::query(
            "INSERT INTO app.audit_log (actor_account_id, action, target_id, payload, reason)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(actor_account_id)
        .bind(action)
        .bind(target_id)
        .bind(payload)
        .bind(reason)
        .execute(executor)
        .await?;
        Ok(())
    }
}
