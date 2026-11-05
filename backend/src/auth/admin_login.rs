// backend/src/auth/admin_login.rs

//! Admin login flow per D-012, D-044, D-045.

use crate::{auth::{attempt_token::AttemptToken, models::AdminLoginRequest}, error::{AppError, AppResult}, state::AppState};
use axum::extract::State;

/// POST /api/v1/admin/login/issue-token
/// Issue attempt token with 4-field form per D-012.
pub async fn issue_attempt_token(
    State(state): State<AppState>,
) -> AppResult<String> {
    let ttl = state.config.auth.attempt_token_ttl_secs;
    let token = AttemptToken::issue(&state.cache, ttl, None).await?;
    Ok(token)
}

/// POST /api/v1/admin/login/password
/// Verify credentials and transition to PASSWORD_OK state per D-012.
pub async fn verify_password(
    State(state): State<AppState>,
    body: AdminLoginRequest,
) -> AppResult<String> {
    // Canonical field-to-credential mapping: server-side only per D-012
    // Never ship in client bundle
    
    // TODO: Verify credentials against database
    // TODO: Transition attempt token to PASSWORD_OK
    // TODO: Return new token
    
    Err(AppError::AuthenticationFailed)
}

/// POST /api/v1/admin/login/initial-code
/// Verify phone-delivered initial code per D-044/D-045.
pub async fn verify_initial_code(
    State(state): State<AppState>,
    // attempt_token from bearer token
    // phone_code from request body
) -> AppResult<()> {
    // TODO: Verify 8-digit code delivered by phone
    // TODO: Force password change on first login
    // TODO: Require WebAuthn enrollment
    
    Err(AppError::AuthenticationFailed)
}

/// POST /api/v1/admin/login/webauthn
/// WebAuthn assertion per D-013.
pub async fn verify_webauthn() -> AppResult<()> {
    // TODO: Verify WebAuthn assertion
    // TODO: Check sign_count
    // TODO: Transition to MFA_OK or SESSION_ISSUED
    
    Err(AppError::AuthenticationFailed)
}

/// POST /api/v1/admin/login/totp
/// TOTP verification per D-047.
pub async fn verify_totp() -> AppResult<()> {
    // TODO: Verify TOTP code
    // TODO: Check for replay
    // TODO: Transition to SESSION_ISSUED
    
    Err(AppError::AuthenticationFailed)
}
