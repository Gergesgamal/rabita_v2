// backend/src/auth/webauthn.rs

//! WebAuthn/FIDO2 registration and assertion per D-013.

use crate::error::{AppError, AppResult};
use webauthn_rs::prelude::*;
use uuid::Uuid;

pub struct WebAuthnManager {
    rp_id: String,
    rp_origin: Url,
}

impl WebAuthnManager {
    pub fn new(rp_id: String, rp_origin_str: String) -> AppResult<Self> {
        let rp_origin = Url::parse(&rp_origin_str)
            .map_err(|_| AppError::BadRequest("Invalid origin URL".to_string()))?;
        
        Ok(Self {
            rp_id,
            rp_origin,
        })
    }

    /// Start registration ceremony.
    pub fn registration_challenge(
        &self,
        user_id: &Uuid,
        user_name: &str,
    ) -> AppResult<(CreationChallengeResponse, Vec<u8>)> {
        let rp = Webauthn::new(
            &self.rp_id,
            &self.rp_origin,
            &[self.rp_origin.clone()],
        )
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("WebAuthn init: {}", e)))?;
        
        let user_id_bytes = user_id.as_bytes().to_vec();
        let (challenge, registration_state) = rp
            .start_passkey_registration(
                user_id_bytes.clone().into(),
                user_name,
                user_name,
                None,
            )
            .map_err(|e| AppError::InternalError(anyhow::anyhow!("WebAuthn challenge: {}", e)))?;
        
        let state_bytes = serde_json::to_vec(&registration_state)?;
        
        Ok((challenge.public_key, state_bytes))
    }

    /// Verify registration response.
    pub fn finish_registration(
        &self,
        challenge_state: &[u8],
        response: &RegisterPublicKeyCredential,
    ) -> AppResult<Vec<u8>> {
        let rp = Webauthn::new(
            &self.rp_id,
            &self.rp_origin,
            &[self.rp_origin.clone()],
        )?;
        
        let reg_state: PasskeyRegistrationState = serde_json::from_slice(challenge_state)?;
        let credential = rp.finish_passkey_registration(response, &reg_state)?;
        
        Ok(credential.cred_public_key.to_vec())
    }
}
