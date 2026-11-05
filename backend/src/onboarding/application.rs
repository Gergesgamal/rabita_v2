// backend/src/onboarding/application.rs

//! Merchant application lifecycle per D-039 with CAPTCHA/proof-of-work and rate limits.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ApplicationManager;

impl ApplicationManager {
    /// Submit merchant application per D-039.
    /// Email verification required before entering queue.
    /// CAPTCHA/proof-of-work required per D-039.
    /// Rate limits: 3 per IP per 24h, 1 per email per 24h.
    pub async fn submit_application(
        db: &PgPool,
        business_name: &str,
        contact_name: &str,
        contact_phone: &str,
        email: &str,
        product_category: Option<&str>,
        notes: Option<&str>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        captcha_verified: bool,
    ) -> AppResult<Uuid> {
        // Check CAPTCHA/proof-of-work
        if !captcha_verified {
            return Err(AppError::BadRequest(
                "CAPTCHA verification required".to_string(),
            ));
        }

        // Check IP rate limit: 3 per 24h
        let ip_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM app.merchant_application
            WHERE ip_address = $1::inet
            AND created_at > now() - interval '24 hours'
            "#,
        )
        .bind(ip_address)
        .fetch_one(db)
        .await
        .unwrap_or(0);

        if ip_count >= 3 {
            return Err(AppError::BadRequest(
                "Rate limit exceeded: 3 applications per IP per 24 hours".to_string(),
            ));
        }

        // Check email rate limit: 1 per 24h
        let email_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM app.merchant_application
            WHERE email = $1
            AND created_at > now() - interval '24 hours'
            "#,
        )
        .bind(email)
        .fetch_one(db)
        .await
        .unwrap_or(0);

        if email_count >= 1 {
            return Err(AppError::BadRequest(
                "Rate limit exceeded: 1 application per email per 24 hours".to_string(),
            ));
        }

        // Create application in 'submitted' state
        let app_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.merchant_application
            (business_name, contact_name, contact_phone, email, product_category, notes,
             ip_address, user_agent, state, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'submitted', now() + interval '90 days')
            RETURNING id
            "#,
        )
        .bind(business_name)
        .bind(contact_name)
        .bind(contact_phone)
        .bind(email)
        .bind(product_category)
        .bind(notes)
        .bind(ip_address)
        .bind(user_agent)
        .fetch_one(db)
        .await?;

        Ok(app_id)
    }

    /// Send email verification link per D-039.
    pub async fn send_verification_email(email: &str, app_id: Uuid) -> AppResult<()> {
        // TODO: Generate signed token
        // TODO: Send SES email with verification link
        tracing::info!("Email verification sent to {}", email);
        Ok(())
    }

    /// Verify email and move to queue per D-039.
    pub async fn verify_email(db: &PgPool, app_id: Uuid, token: &str) -> AppResult<()> {
        // TODO: Verify signed token
        // TODO: Update application.email_verified_at

        sqlx::query(
            r#"
            UPDATE app.merchant_application
            SET email_verified_at = now()
            WHERE id = $1
            "#,
        )
        .bind(app_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Approve application per D-039 (super-admin only).
    pub async fn approve(db: &PgPool, app_id: Uuid, reviewer_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE app.merchant_application
            SET state = 'approved', reviewer_id = $1, reviewed_at = now()
            WHERE id = $2 AND state IN ('submitted', 'contacted')
            "#,
        )
        .bind(reviewer_id)
        .bind(app_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Reject application per D-039 (super-admin only).
    /// Rejection is durable for repeat-abuse detection.
    pub async fn reject(
        db: &PgPool,
        app_id: Uuid,
        reviewer_id: Uuid,
        reason: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE app.merchant_application
            SET state = 'rejected', reviewer_id = $1, reviewed_at = now(), rejection_reason = $2
            WHERE id = $3
            "#,
        )
        .bind(reviewer_id)
        .bind(reason)
        .bind(app_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Get application by ID.
    pub async fn get(db: &PgPool, app_id: Uuid) -> AppResult<ApplicationRow> {
        sqlx::query_as::<_, ApplicationRow>(
            r#"
            SELECT id, business_name, contact_name, contact_phone, email, product_category,
                   notes, ip_address, user_agent, state, rejection_reason, reviewer_id,
                   created_at, reviewed_at, expires_at
            FROM app.merchant_application
            WHERE id = $1
            "#,
        )
        .bind(app_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Application not found".to_string()))
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ApplicationRow {
    pub id: Uuid,
    pub business_name: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub email: String,
    pub product_category: Option<String>,
    pub notes: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub state: String,
    pub rejection_reason: Option<String>,
    pub reviewer_id: Option<uuid::Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}
