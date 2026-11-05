// backend/src/authorization/check.rs

//! Authorization check: (account_id, current_context, capability) per D-004.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

/// Per-request authorization context.
pub struct AuthorizationCheck {
    pub account_id: Uuid,
    pub context_id: Uuid,
    pub capability: String,
}

impl AuthorizationCheck {
    /// Check authorization in database: does account have capability in context?
    /// Per D-004, this checks (account_id, context_id) + capability, not account_id alone.
    pub async fn execute(&self, db: &PgPool) -> AppResult<()> {
        let has_capability = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM app.account_permission ap
                JOIN app.permission_catalog pc ON ap.capability_id = pc.id
                WHERE ap.account_id = $1
                AND ap.context_id = $2
                AND pc.capability_name = $3
                AND (ap.expires_at IS NULL OR ap.expires_at > now())
                AND ap.created_at <= now()
            )
            "#,
        )
        .bind(self.account_id)
        .bind(self.context_id)
        .bind(&self.capability)
        .fetch_one(db)
        .await?;

        if has_capability {
            Ok(())
        } else {
            Err(AppError::Forbidden(format!(
                "Missing capability: {}",
                self.capability
            )))
        }
    }

    /// Check without context_id (for testing privilege bleed vulnerability).
    /// This is DELIBERATELY BROKEN and used in tests to prove the vulnerability.
    #[allow(dead_code)]
    async fn execute_broken_no_context(&self, db: &PgPool) -> AppResult<()> {
        // BUG: This only checks account_id and capability, ignoring context_id.
        // This is intentionally broken for testing privilege bleed.
        let has_capability = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM app.account_permission ap
                JOIN app.permission_catalog pc ON ap.capability_id = pc.id
                WHERE ap.account_id = $1
                AND pc.capability_name = $2
                AND (ap.expires_at IS NULL OR ap.expires_at > now())
            )
            "#,
        )
        .bind(self.account_id)
        .bind(&self.capability)
        .fetch_one(db)
        .await?;

        if has_capability {
            Ok(())
        } else {
            Err(AppError::Forbidden(format!(
                "Missing capability: {}",
                self.capability
            )))
        }
    }
}
