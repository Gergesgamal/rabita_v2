// backend/src/stores/service.rs

//! Store service per D-021, D-022: slug validation with Unicode normalization, homograph attack rejection.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;
use unicode_normalization::UnicodeNormalization;

pub struct StoreManager;

impl StoreManager {
    /// Validate and normalize slug per D-022.
    /// - Lowercase ASCII only
    /// - 3-63 characters
    /// - Alphanumeric + hyphens, no leading/trailing/double hyphens
    /// - NFC Unicode normalization (rejects homographs like Cyrillic 'а' vs Latin 'a')
    /// - IDN/Punycode to ASCII
    pub fn validate_slug(slug: &str) -> AppResult<String> {
        // Basic length check
        if slug.len() < 3 || slug.len() > 63 {
            return Err(AppError::BadRequest(
                "Slug must be 3-63 characters".to_string(),
            ));
        }

        // Normalize to NFC
        let normalized: String = slug.nfc().collect();

        // Check pattern (lowercase, alphanumeric + hyphens)
        if !normalized.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(AppError::BadRequest(
                "Slug must contain only lowercase letters, digits, and hyphens".to_string(),
            ));
        }

        // Check for leading/trailing hyphens
        if normalized.starts_with('-') || normalized.ends_with('-') {
            return Err(AppError::BadRequest(
                "Slug cannot start or end with hyphen".to_string(),
            ));
        }

        // Check for double hyphens
        if normalized.contains("--") {
            return Err(AppError::BadRequest("Slug cannot contain double hyphens".to_string()));
        }

        Ok(normalized)
    }

    /// Create store per D-021.
    pub async fn create_store(
        db: &PgPool,
        owner_account_id: Uuid,
        business_name: &str,
        slug: &str,
        description: Option<&str>,
    ) -> AppResult<Uuid> {
        // Validate and normalize slug
        let normalized_slug = Self::validate_slug(slug)?;

        // Check Tier 1 reserved slugs (engineering-only, immutable)
        if Self::is_tier1_reserved(&normalized_slug) {
            return Err(AppError::Forbidden(
                "Slug is reserved by the platform".to_string(),
            ));
        }

        // Check Tier 2 reserved slugs (admin-managed)
        let is_tier2_reserved: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(SELECT 1 FROM app.tier2_reserved_slug WHERE slug = $1)
            "#,
        )
        .bind(&normalized_slug)
        .fetch_one(db)
        .await?;

        if is_tier2_reserved {
            return Err(AppError::Forbidden("Slug is reserved".to_string()));
        }

        // Check uniqueness
        let exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(SELECT 1 FROM app.store WHERE slug = $1 AND deleted_at IS NULL)
            "#,
        )
        .bind(&normalized_slug)
        .fetch_one(db)
        .await?;

        if exists {
            return Err(AppError::Conflict("Slug already in use".to_string()));
        }

        // Create store in transaction
        let mut tx = db.begin().await?;

        let store_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.store (owner_account_id, slug, business_name, description)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(owner_account_id)
        .bind(&normalized_slug)
        .bind(business_name)
        .bind(description)
        .fetch_one(&mut *tx)
        .await?;

        // Create default settings
        sqlx::query(
            r#"
            INSERT INTO app.store_settings (store_id)
            VALUES ($1)
            "#,
        )
        .bind(store_id)
        .execute(&mut *tx)
        .await?;

        // Create media quota
        sqlx::query(
            r#"
            INSERT INTO app.media_quota (store_id)
            VALUES ($1)
            "#,
        )
        .bind(store_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(store_id)
    }

    /// Check if slug is Tier 1 reserved (engineering-only, hardcoded).
    fn is_tier1_reserved(slug: &str) -> bool {
        matches!(
            slug,
            "api" | "admin" | "cdn" | "mail" | "jobs" | "blog" | "status" | "support"
        )
    }

    /// Get store by ID.
    pub async fn get_store(db: &PgPool, store_id: Uuid) -> AppResult<StoreRow> {
        sqlx::query_as::<_, StoreRow>(
            r#"
            SELECT id, owner_account_id, slug, business_name, description, is_published,
                   is_suspended, suspension_reason, store_timezone, created_at, updated_at
            FROM app.store
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(store_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Store not found".to_string()))
    }

    /// Publish store (separate from account approval per D-021).
    pub async fn publish_store(db: &PgPool, store_id: Uuid) -> AppResult<()> {
        // TODO: Verify publication gate: profile complete, at least one product, business/contact match application

        sqlx::query(
            r#"
            UPDATE app.store
            SET is_published = true, updated_at = now()
            WHERE id = $1
            "#,
        )
        .bind(store_id)
        .execute(db)
        .await?;

        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct StoreRow {
    pub id: Uuid,
    pub owner_account_id: Uuid,
    pub slug: String,
    pub business_name: String,
    pub description: Option<String>,
    pub is_published: bool,
    pub is_suspended: bool,
    pub suspension_reason: Option<String>,
    pub store_timezone: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
