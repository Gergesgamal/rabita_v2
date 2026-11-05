// backend/src/onboarding/provisioning.rs

//! Manual credential provisioning per D-045: phone-delivered initial code, 24h TTL.

use crate::error::{AppError, AppResult};
use crate::auth::password::PasswordManager;
use sqlx::PgPool;
use uuid::Uuid;
use rand::Rng;

pub struct ProvisioningManager;

impl ProvisioningManager {
    /// Generate merchant identifier and temporary password per D-045.
    /// Identifier format: "mer-" + 12 random base32 chars (excluding ambiguous L/I/O).
    /// Password: 12-16 chars from base32 alphabet, no ambiguous chars.
    pub fn generate_credentials() -> (String, String) {
        const ALPHABET: &[u8] = b"023456789ABCDEFGHJKMNPQRSTVWXYZ"; // No L, I, O, U
        let mut rng = rand::thread_rng();

        // Generate identifier: "mer-" + 12 random chars
        let mut identifier = String::from("mer-");
        for _ in 0..12 {
            let idx = rng.gen_range(0..ALPHABET.len());
            identifier.push(ALPHABET[idx] as char);
        }

        // Generate temp password: 12-16 chars
        let password_len = rng.gen_range(12..=16);
        let mut password = String::new();
        for _ in 0..password_len {
            let idx = rng.gen_range(0..ALPHABET.len());
            password.push(ALPHABET[idx] as char);
        }

        (identifier, password)
    }

    /// Generate initial 8-digit code to be delivered by phone per D-045.
    /// 15-minute TTL, single-use, burned on first attempt.
    pub fn generate_initial_code() -> String {
        let mut rng = rand::thread_rng();
        let code = rng.gen_range(10000000..=99999999);
        code.to_string()
    }

    /// Create account from approved application per D-045.
    /// Requires manual phone call; no automated notification per D-045.
    /// Credentials delivered during phone call only.
    pub async fn provision_merchant(
        db: &PgPool,
        app_id: Uuid,
        provisioner_id: Uuid,
    ) -> AppResult<(String, String, String)> {
        // Verify application is approved
        let app: super::application::ApplicationRow =
            super::application::ApplicationManager::get(db, app_id).await?;

        if app.state != "approved" {
            return Err(AppError::BadRequest(
                "Application must be approved before provisioning".to_string(),
            ));
        }

        // Generate identifier, password, initial code
        let (identifier, temp_password) = Self::generate_credentials();
        let initial_code = Self::generate_initial_code();

        // Hash password
        let password_hash = PasswordManager::hash_password(&temp_password)?;

        // Hash initial code for storage
        let code_hash = PasswordManager::hash_password(&initial_code)?;

        // Create account within transaction
        let mut tx = db.begin().await?;

        let account_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.account (identifier, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(&identifier)
        .bind(&app.email)
        .bind(&password_hash)
        .fetch_one(&mut *tx)
        .await?;

        // Create merchant role binding
        sqlx::query(
            r#"
            INSERT INTO app.account_role (account_id, role_name, context_id)
            VALUES ($1, 'merchant', gen_random_uuid())
            "#,
        )
        .bind(account_id)
        .execute(&mut *tx)
        .await?;

        // Store initial code (in production, KMS-encrypted)
        sqlx::query(
            r#"
            INSERT INTO app.recovery_code (account_id, code_hash, used_at)
            VALUES ($1, $2, NULL)
            "#,
        )
        .bind(account_id)
        .bind(&code_hash)
        .execute(&mut *tx)
        .await?;

        // Update application state to 'invited'
        sqlx::query(
            r#"
            UPDATE app.merchant_application
            SET state = 'invited'
            WHERE id = $1
            "#,
        )
        .bind(app_id)
        .execute(&mut *tx)
        .await?;

        // Audit provisioning
        sqlx::query(
            r#"
            INSERT INTO app.audit_log
            (event_type, actor_id, actor_type, target_id, target_type, reason, created_at)
            VALUES ('account_provisioned', $1, 'account', $2, 'account', 'Merchant provisioned', now())
            "#,
        )
        .bind(provisioner_id)
        .bind(account_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // Log (not credentials!) that provisioning succeeded
        tracing::info!(
            "Merchant account provisioned: {} for application {}",
            identifier,
            app_id
        );

        Ok((identifier, temp_password, initial_code))
    }

    /// Store initial code in session for verification per D-045.
    /// Code is 8 digits, 15-minute TTL, single-use.
    pub async fn issue_initial_code_challenge(
        cache: &redis::aio::ConnectionManager,
        account_id: Uuid,
    ) -> AppResult<()> {
        let key = format!("initial_code:{}", account_id);
        let mut conn = cache.clone();

        // Code already stored in recovery_code table; this is just marking it active in Valkey
        redis::cmd("SET")
            .arg(&key)
            .arg("pending")
            .arg("EX")
            .arg(900) // 15 minutes
            .query_async::<_, ()>(&mut conn)
            .await?;

        Ok(())
    }
}
