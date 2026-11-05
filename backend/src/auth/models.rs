// backend/src/auth/models.rs

//! Authentication request/response DTOs.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub user_code: String,
    pub password: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminLoginInitialCodeRequest {
    pub phone_code: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminLoginMfaRequest {
    pub method: String, // 'webauthn' or 'totp'
    pub response: String, // Serialized WebAuthn assertion or TOTP code
}

#[derive(Debug, Serialize)]
pub struct AdminLoginResponse {
    pub session_token: String,
}

#[derive(Debug, Deserialize)]
pub struct MerchantLoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct MerchantMfaRequest {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct MerchantLoginResponse {
    pub session_token: String,
    pub requires_password_change: bool,
    pub requires_enrollment: bool,
}
