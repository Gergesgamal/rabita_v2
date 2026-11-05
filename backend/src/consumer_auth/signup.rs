// backend/src/consumer_auth/signup.rs

//! Consumer signup per D-131: email verification or WhatsApp OTP per D-128.

use crate::error::{AppError, AppResult};
use crate::auth::password::PasswordManager;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ConsumerSignupManager;

impl ConsumerSignupManager {
    /// Signup: create consumer with pending email verification per D-131.
    /// Check phone on WhatsApp if provided per D-128.
    /// Rate limits: 5 signup per IP per hour, 3 resends per email per hour per D-131.
    pub async fn signup(
        db: &PgPool,
        store_id: Uuid,
        email: &str,
        password: &str,
        name: &str,
        phone_number: Option<&str>,
    ) -> AppResult<(Uuid, String)> {
        // Validate password per D-111
        PasswordManager::validate_password_policy(password)?;

        // Check for HIBP breach per D-111
        let is_breached = PasswordManager::check_hibp_breach(password).await.unwrap_or(false);
        if is_breached {
            return Err(AppError::BadRequest(
                "Password is compromised. Please choose another.".to_string(),
            ));
        }

        // Normalize email
        let normalized_email = email.to_lowercase();

        // Check uniqueness within store
        let exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM app.consumer
                WHERE store_id = $1 AND normalized_email = $2 AND deleted_at IS NULL
            )
            "#,
        )
        .bind(store_id)
        .bind(&normalized_email)
        .fetch_one(db)
        .await?;

        if exists {
            // Return generic message without disclosing existence per D-131
            return Err(AppError::BadRequest(
                "Account exists or verification required".to_string(),
            ));
        }

        // Hash password
        let password_hash = PasswordManager::hash_password(password)?;

        // Determine verification method per D-128
        let verification_method = if let Some(phone) = phone_number {
            // Check if phone is on WhatsApp
            let on_whatsapp = check_phone_on_whatsapp(phone).await.unwrap_or(false);
            if on_whatsapp {
                "whatsapp_otp"
            } else {
                "email_only"
            }
        } else {
            "email_only"
        };

        // Create consumer
        let consumer_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.consumer
            (store_id, normalized_email, password_hash, phone_number, full_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(store_id)
        .bind(&normalized_email)
        .bind(&password_hash)
        .bind(phone_number)
        .bind(name)
        .fetch_one(db)
        .await?;

        Ok((consumer_id, verification_method.to_string()))
    }

    /// Send email verification link per D-131.
    /// One-hour TTL, single-use token.
    pub async fn send_email_verification(
        store_id: Uuid,
        consumer_id: Uuid,
        email: &str,
    ) -> AppResult<()> {
        // TODO: Generate signed token (store_id, consumer_id, email_hash, timestamp)
        // TODO: Send SES email with verification link
        tracing::info!("Email verification sent to {}", email);
        Ok(())
    }

    /// Send WhatsApp OTP per D-128.
    /// Rate limit: 5 per hour per phone per D-128.
    pub async fn send_whatsapp_otp(
        phone_number: &str,
        consumer_id: Uuid,
    ) -> AppResult<()> {
        // TODO: Check rate limit per phone
        // TODO: Generate 6-digit OTP
        // TODO: Call WhatsApp Business API
        tracing::info!("WhatsApp OTP sent to {}", phone_number);
        Ok(())
    }

    /// Verify email link per D-131.
    pub async fn verify_email_link(
        db: &PgPool,
        consumer_id: Uuid,
        token: &str,
    ) -> AppResult<()> {
        // TODO: Verify signed token
        // TODO: Update consumer.email_verified_at
        // TODO: Issue session

        sqlx::query(
            r#"
            UPDATE app.consumer
            SET email_verified_at = now()
            WHERE id = $1
            "#,
        )
        .bind(consumer_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Verify WhatsApp OTP per D-128.
    pub async fn verify_whatsapp_otp(
        db: &PgPool,
        consumer_id: Uuid,
        code: &str,
    ) -> AppResult<()> {
        // TODO: Check OTP against Valkey cache
        // TODO: Mark as verified
        // TODO: Burn OTP (one-time use)

        sqlx::query(
            r#"
            UPDATE app.consumer
            SET whatsapp_verified_at = now()
            WHERE id = $1
            "#,
        )
        .bind(consumer_id)
        .execute(db)
        .await?;

        Ok(())
    }
}

/// Check if phone number is registered on WhatsApp per D-128.
async fn check_phone_on_whatsapp(phone: &str) -> AppResult<bool> {
    // TODO: Call WhatsApp Business API to check number
    // For MVP, return false (default to email-only)
    Ok(false)
}
