// backend/src/auth/totp.rs

//! TOTP enrollment and verification with replay prevention per D-047.

use crate::error::{AppError, AppResult};
use totp_rs::{TOTP, Secret, Algorithm};

pub struct TOTPManager;

impl TOTPManager {
    /// Generate new TOTP secret for enrollment.
    pub fn generate_secret(account_id: &str) -> AppResult<(String, String)> {
        let secret = Secret::generate_secret();
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_bytes()?,
            Some("Rabita".to_string()),
            account_id.to_string(),
        )?;
        
        let url = totp.get_url();
        Ok((secret.to_string(), url))
    }

    /// Verify TOTP code with replay prevention per D-047.
    pub fn verify_code(
        secret: &str,
        code: &str,
        last_accepted_step: Option<i64>,
    ) -> AppResult<(bool, i64)> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Encoded(secret.to_string()).to_bytes()?,
            Some("Rabita".to_string()),
            "user".to_string(),
        )?;
        
        match totp.check_current(code) {
            Ok(()) => {
                let current_step = (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs() / 30) as i64;
                
                // Reject if same step as last accepted
                if let Some(last_step) = last_accepted_step {
                    if current_step == last_step {
                        return Err(AppError::BadRequest("Code already used".to_string()));
                    }
                }
                
                Ok((true, current_step))
            }
            Err(_) => Ok((false, 0)),
        }
    }
}
