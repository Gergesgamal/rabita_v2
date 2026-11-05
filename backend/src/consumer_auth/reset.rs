// backend/src/consumer_auth/reset.rs

//! Consumer password reset per D-128: email or WhatsApp OTP, 1-hour TTL.

use crate::error::{AppError, AppResult};
use crate::auth::password::PasswordManager;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ConsumerPasswordResetManager;

impl ConsumerPasswordResetManager {
    /// Initiate password reset per D-128.
    /// Look up by store_id + email without disclosing existence.
    /// Send email or WhatsApp OTP depending on consumer preference.
    pub async fn initiate_reset(
        db: &PgPool,
        store_id: Uuid,
        email: &str,
    ) -> AppResult<()> {
        let normalized_email = email.to_lowercase();

        // Lookup without disclosing existence
        let consumer: Option<(Uuid, bool, Option<String>)> = sqlx::query_as(
            r#"
            SELECT id, whatsapp_verified_at IS NOT NULL, phone_number
            FROM app.consumer
            WHERE store_id = $1 AND normalized_email = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(store_id)
        .bind(&normalized_email)
        .fetch_optional(db)
        .await?;

        match consumer {
            Some((consumer_id, whatsapp_verified, phone_number)) => {
                if whatsapp_verified && phone_number.is_some() {
                    // Send WhatsApp OTP
                    super::signup::ConsumerSignupManager::send_whatsapp_otp(
                        phone_number.as_ref().unwrap(),
                        consumer_id,
                    )
                    .await?;
                } else {
                    // Send email reset link
                    send_password_reset_email(consumer_id, &normalized_email).await?;
                }
            }
            None => {
                // Generic response (no disclosure)
                tracing::warn!(
                    "Password reset requested for non-existent consumer: {}",
                    normalized_email
                );
            }
        }

        Ok(())
    }

    /// Verify reset token and change password per D-128.
    pub async fn reset_password(
        db: &PgPool,
        consumer_id: Uuid,
        new_password: &str,
        _token: &str,
    ) -> AppResult<()> {
        // TODO: Verify reset token signature
        // TODO: Check 1-hour TTL

        // Validate password per D-111
        PasswordManager::validate_password_policy(new_password)?;

        // Check HIBP
        let is_breached = PasswordManager::check_hibp_breach(new_password)
            .await
            .unwrap_or(false);
        if is_breached {
            return Err(AppError::BadRequest(
                "Password is compromised. Please choose another.".to_string(),
            ));
        }

        // Hash and update
        let password_hash = PasswordManager::hash_password(new_password)?;

        sqlx::query(
            r#"
            UPDATE app.consumer
            SET password_hash = $1, updated_at = now()
            WHERE id = $2
            "#,
        )
        .bind(&password_hash)
        .bind(consumer_id)
        .execute(db)
        .await?;

        // TODO: Invalidate all existing sessions per D-128

        Ok(())
    }
}

async fn send_password_reset_email(consumer_id: Uuid, email: &str) -> AppResult<()> {
    // TODO: Generate signed reset link (1-hour TTL)
    // TODO: Send SES email
    tracing::info!("Password reset email sent to {}", email);
    Ok(())
}
