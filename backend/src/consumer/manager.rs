use crate::error::{AppError, ErrorCode};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct ConsumerManager;

impl ConsumerManager {
    /// Get or create consumer account (store-scoped per D-021)
    pub async fn get_or_create_consumer(
        db: &PgPool,
        store_id: Uuid,
        email: &str,
        name: &str,
    ) -> Result<ConsumerAccount, AppError> {
        // Normalize email
        let normalized_email = email.to_lowercase();

        // Try to get existing
        let existing: Option<ConsumerAccount> = sqlx::query_as(
            "SELECT * FROM app.consumer WHERE store_id = $1 AND email = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&normalized_email)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if let Some(consumer) = existing {
            return Ok(consumer);
        }

        // Create new consumer
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.consumer (id, store_id, email, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $5)
            "#,
        )
        .bind(&id)
        .bind(&store_id)
        .bind(&normalized_email)
        .bind(name)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Create default consent
        sqlx::query(
            "INSERT INTO app.consumer_consent (consumer_id, marketing_emails, whatsapp_messages, analytics, third_party, updated_at) VALUES ($1, false, false, false, false, $2)",
        )
        .bind(&id)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Self::get_consumer(db, store_id, id).await
    }

    /// Get consumer account
    pub async fn get_consumer(
        db: &PgPool,
        store_id: Uuid,
        consumer_id: Uuid,
    ) -> Result<ConsumerAccount, AppError> {
        sqlx::query_as::<_, ConsumerAccount>(
            "SELECT * FROM app.consumer WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL",
        )
        .bind(&consumer_id)
        .bind(&store_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Consumer not found"))
    }

    /// Update profile
    pub async fn update_profile(
        db: &PgPool,
        store_id: Uuid,
        consumer_id: Uuid,
        req: UpdateProfileRequest,
    ) -> Result<ConsumerAccount, AppError> {
        let _consumer = Self::get_consumer(db, store_id, consumer_id).await?;

        let now = Utc::now();

        sqlx::query(
            "UPDATE app.consumer SET name = COALESCE($1, name), phone = COALESCE($2, phone), updated_at = $3 WHERE id = $4",
        )
        .bind(&req.name)
        .bind(&req.phone)
        .bind(&now)
        .bind(&consumer_id)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Self::get_consumer(db, store_id, consumer_id).await
    }

    /// Add address
    pub async fn add_address(
        db: &PgPool,
        store_id: Uuid,
        consumer_id: Uuid,
        req: AddAddressRequest,
    ) -> Result<ConsumerAddress, AppError> {
        let _consumer = Self::get_consumer(db, store_id, consumer_id).await?;

        let id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO app.consumer_address (id, consumer_id, governorate, county, district, address, phone) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&id)
        .bind(&consumer_id)
        .bind(&req.governorate)
        .bind(&req.county)
        .bind(&req.district)
        .bind(&req.address)
        .bind(&req.phone)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        sqlx::query_as::<_, ConsumerAddress>(
            "SELECT * FROM app.consumer_address WHERE id = $1",
        )
        .bind(&id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Update consent
    pub async fn update_consent(
        db: &PgPool,
        store_id: Uuid,
        consumer_id: Uuid,
        req: UpdateConsentRequest,
    ) -> Result<ConsumerConsent, AppError> {
        let _consumer = Self::get_consumer(db, store_id, consumer_id).await?;

        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE app.consumer_consent
            SET marketing_emails = COALESCE($1, marketing_emails),
                whatsapp_messages = COALESCE($2, whatsapp_messages),
                analytics = COALESCE($3, analytics),
                third_party = COALESCE($4, third_party),
                updated_at = $5
            WHERE consumer_id = $6
            "#,
        )
        .bind(req.marketing_emails)
        .bind(req.whatsapp_messages)
        .bind(req.analytics)
        .bind(req.third_party)
        .bind(&now)
        .bind(&consumer_id)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        sqlx::query_as::<_, ConsumerConsent>(
            "SELECT * FROM app.consumer_consent WHERE consumer_id = $1",
        )
        .bind(&consumer_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Soft delete consumer per D-010 (30-day recovery window)
    pub async fn request_deletion(
        db: &PgPool,
        store_id: Uuid,
        consumer_id: Uuid,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        let recovery_deadline = now + chrono::Duration::days(30);

        sqlx::query(
            "UPDATE app.consumer SET deleted_at = $1 WHERE id = $2 AND store_id = $3",
        )
        .bind(&now)
        .bind(&consumer_id)
        .bind(&store_id)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // TODO: Schedule anonymization after 30 days

        Ok(())
    }
}
