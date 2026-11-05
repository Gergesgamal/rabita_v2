use unicode_normalization::UnicodeNormalization;
use crate::error::{AppError, ErrorCode};
use crate::audit::AuditLogger;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct StoreManager;

impl StoreManager {
    /// Create a store. Validates slug uniqueness, normalizes to NFC+lowercase.
    /// Per D-034: account_id comes from context, never user input.
    pub async fn create_store(
        pool: &PgPool,
        account_id: Uuid,
        req: CreateStoreRequest,
    ) -> Result<Store, AppError> {
        // Normalize slug: lowercase, NFC
        let normalized_slug = req.slug.to_lowercase().nfc()
        .collect::<String>();

        // Validate: 3-63 chars, alphanumeric + hyphens, no leading/trailing/double hyphens
        if normalized_slug.len() < 3 || normalized_slug.len() > 63 {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "Store slug must be 3-63 characters",
            ));
        }
        if !normalized_slug
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-')
            || normalized_slug.starts_with('-')
            || normalized_slug.ends_with('-')
            || normalized_slug.contains("--")
        {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "Slug must be alphanumeric with hyphens, no leading/trailing/double hyphens",
            ));
        }

        // Check if slug is reserved (Tier 1 or Tier 2)
        let reserved: Option<ReservedSlug> = sqlx::query_as(
            "SELECT id, slug, tier, reason, created_at, created_by FROM app.tier2_reserved_slug WHERE slug = $1",
        )
        .bind(&normalized_slug)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if reserved.is_some() {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Slug is reserved",
            ));
        }

        // Check uniqueness
        let existing: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM app.store WHERE slug = $1 AND deleted_at IS NULL")
            .bind(&normalized_slug)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::database_error(&e))?;

        if existing.is_some() {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Slug already in use",
            ));
        }

        // Create store in transaction
        let mut tx = pool.begin().await.map_err(|e| AppError::database_error(&e))?;

        let store_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.store (id, account_id, slug, name, description, is_published, is_suspended, contact_email, contact_phone, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, false, false, $6, $7, $8, $8)
            "#,
        )
        .bind(&store_id)
        .bind(&account_id)
        .bind(&normalized_slug)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.contact_email)
        .bind(&req.contact_phone)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") {
                AppError::new(ErrorCode::Conflict, "Slug already in use")
            } else {
                AppError::database_error(&e)
            }
        })?;

        // Create default settings
        sqlx::query(
            r#"
            INSERT INTO app.store_settings (store_id, ticket_lifetime_days, tax_rate_bp, cancellation_window_seconds, allow_guest_checkout, updated_at)
            VALUES ($1, 14, 1400, 0, true, $2)
            "#,
        )
        .bind(&store_id)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Audit
        AuditLogger::log_action(
            &mut *tx,
            account_id,
            "store.create",
            Some(store_id),
            serde_json::json!({ "slug": normalized_slug, "name": req.name }),
            None,
        )
        .await?;

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        // Fetch and return
        Self::get_store(pool, account_id, store_id).await
    }

    /// Get store by ID. Validates account ownership.
    pub async fn get_store(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
    ) -> Result<Store, AppError> {
        sqlx::query_as::<_, Store>(
            "SELECT * FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))
    }

    /// List stores for account.
    pub async fn list_stores(
        pool: &PgPool,
        account_id: Uuid,
    ) -> Result<Vec<Store>, AppError> {
        sqlx::query_as::<_, Store>(
            "SELECT * FROM app.store WHERE account_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC",
        )
        .bind(&account_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Update store.
    pub async fn update_store(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        req: UpdateStoreRequest,
    ) -> Result<Store, AppError> {
        let store = Self::get_store(pool, account_id, store_id).await?;
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE app.store
            SET name = COALESCE($1, name),
                description = COALESCE($2, description),
                contact_email = COALESCE($3, contact_email),
                contact_phone = COALESCE($4, contact_phone),
                accent_color = COALESCE($5, accent_color),
                updated_at = $6
            WHERE id = $7
            "#,
        )
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.contact_email)
        .bind(&req.contact_phone)
        .bind(&req.accent_color)
        .bind(&now)
        .bind(&store_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            "store.update",
            Some(store_id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_store(pool, account_id, store_id).await
    }

    /// Update store settings.
    pub async fn update_settings(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        req: UpdateStoreSettingsRequest,
    ) -> Result<StoreSettings, AppError> {
        // Validate store ownership first
        let _ = Self::get_store(pool, account_id, store_id).await?;

        // Validate ranges
        if let Some(days) = req.ticket_lifetime_days {
            if days < 10 || days > 90 {
                return Err(AppError::new(
                    ErrorCode::InvalidInput,
                    "Ticket lifetime must be 10-90 days",
                ));
            }
        }

        if let Some(bp) = req.tax_rate_bp {
            if bp < 0 || bp > 10000 {
                return Err(AppError::new(
                    ErrorCode::InvalidInput,
                    "Tax rate must be 0-10000 basis points",
                ));
            }
        }

        if let Some(seconds) = req.cancellation_window_seconds {
            if seconds < 0 {
                return Err(AppError::new(
                    ErrorCode::InvalidInput,
                    "Cancellation window cannot be negative",
                ));
            }
        }

        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE app.store_settings
            SET ticket_lifetime_days = COALESCE($1, ticket_lifetime_days),
                tax_rate_bp = COALESCE($2, tax_rate_bp),
                cancellation_window_seconds = COALESCE($3, cancellation_window_seconds),
                allow_guest_checkout = COALESCE($4, allow_guest_checkout),
                updated_at = $5
            WHERE store_id = $6
            "#,
        )
        .bind(&req.ticket_lifetime_days)
        .bind(&req.tax_rate_bp)
        .bind(&req.cancellation_window_seconds)
        .bind(&req.allow_guest_checkout)
        .bind(&now)
        .bind(&store_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            "store_settings.update",
            Some(store_id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        sqlx::query_as::<_, StoreSettings>(
            "SELECT * FROM app.store_settings WHERE store_id = $1",
        )
        .bind(&store_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Suspend store. Super-admin only per D-123.
    pub async fn suspend_store(
        pool: &PgPool,
        admin_id: Uuid,
        store_id: Uuid,
        req: SuspendStoreRequest,
    ) -> Result<Store, AppError> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE app.store
            SET is_suspended = true, suspension_reason = $1, updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(&req.reason)
        .bind(&now)
        .bind(&store_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Log audit
        AuditLogger::log_action(
            pool,
            admin_id,
            "store.suspend",
            Some(store_id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        // TODO: invalidate employee sessions for this store

        // Fetch and return
        sqlx::query_as::<_, Store>(
            "SELECT * FROM app.store WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Publish/unpublish store.
    pub async fn set_publication(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        publish: bool,
    ) -> Result<Store, AppError> {
        let store = Self::get_store(pool, account_id, store_id).await?;

        // If publishing, require complete profile + at least one published product
        if publish && !store.is_published {
            // Check preconditions
            let has_products: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM app.product WHERE store_id = $1 AND is_published = true",
            )
            .bind(&store_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            if has_products.0 == 0 {
                return Err(AppError::new(
                    ErrorCode::InvalidState,
                    "Store must have at least one published product to publish",
                ));
            }
        }

        let now = Utc::now();

        sqlx::query(
            "UPDATE app.store SET is_published = $1, updated_at = $2 WHERE id = $3",
        )
        .bind(publish)
        .bind(&now)
        .bind(&store_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            if publish { "store.publish" } else { "store.unpublish" },
            Some(store_id),
            serde_json::json!({}),
            None,
        )
        .await?;

        Self::get_store(pool, account_id, store_id).await
    }
}
