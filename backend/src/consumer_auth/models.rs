// backend/src/consumer_auth/models.rs

//! Consumer auth models.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ConsumerSignupRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConsumerSignupResponse {
    pub email_verified: bool,
    pub verification_sent_via: String, // 'email' or 'whatsapp'
}

#[derive(Debug, Deserialize)]
pub struct ConsumerVerifyRequest {
    pub verification_method: String, // 'email_link' or 'whatsapp_otp'
    pub token_or_code: String,
}

#[derive(Debug, Deserialize)]
pub struct ConsumerLoginRequest {
    pub store_slug: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ConsumerLoginResponse {
    pub session_token: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetRequest {
    pub store_slug: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetConfirmRequest {
    pub reset_token: String,
    pub new_password: String,
}
