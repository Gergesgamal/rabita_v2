// backend/src/authorization/context.rs

//! Context switching and claim reissue per D-004, D-023.

use crate::error::{AppError, AppResult};
use crate::auth::session::SessionClaim;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ContextManager;

impl ContextManager {
    /// Switch context and reissue session claim per D-004.
    /// Atomically reads live bindings and permission version.
    pub async fn switch_context(
        db: &PgPool,
        account_id: Uuid,
        new_context_id: Uuid,
    ) -> AppResult<SessionClaim> {
        // Verify the new context binding exists
        let (role, permission_version): (String, i64) = sqlx::query_as(
            r#"
            SELECT ar.role_name, COALESCE(MAX(ap.version), 1)
            FROM app.account_role ar
            LEFT JOIN app.account_permission ap ON ap.account_id = ar.account_id
            AND ar.context_id = ap.context_id
            WHERE ar.account_id = $1
            AND ar.context_id = $2
            AND ar.deleted_at IS NULL
            GROUP BY ar.role_name
            "#,
        )
        .bind(account_id)
        .bind(new_context_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| {
            AppError::Forbidden("Context binding not found or revoked".to_string())
        })?;

        // Create new claim with updated context
        let now = chrono::Utc::now();
        let expires = now + chrono::Duration::days(30);

        Ok(SessionClaim {
            consumer_id: None,
            session_id: Uuid::new_v4(),
            account_id,
            context_id: new_context_id,
            role,
            permission_version,
            issued_at: now,
            expires_at: expires,
        })
    }

    /// Detect permission revocation via version mismatch per D-004.
    pub async fn check_permission_version(
        db: &PgPool,
        account_id: Uuid,
        context_id: Uuid,
        claimed_version: i64,
    ) -> AppResult<()> {
        let actual_version: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(MAX(version), 1)
            FROM app.account_permission
            WHERE account_id = $1 AND context_id = $2
            "#,
        )
        .bind(account_id)
        .bind(context_id)
        .fetch_one(db)
        .await?;

        if claimed_version < actual_version {
            // Permissions were revoked
            return Err(AppError::Unauthorized);
        }

        Ok(())
    }
}
