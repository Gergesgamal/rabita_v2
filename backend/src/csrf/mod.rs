// backend/src/csrf/mod.rs

//! CSRF protection per D-110: double-submit token on consumer (SameSite=Lax) and admin (SameSite=Strict).

use crate::error::{AppError, AppResult};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use uuid::Uuid;

/// CSRF token extractor from X-CSRF-Token header.
#[derive(Debug, Clone)]
pub struct CsrfToken(pub String);

#[async_trait]
impl FromRequestParts<crate::state::AppState> for CsrfToken {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &crate::state::AppState,
    ) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("x-csrf-token")
            .and_then(|h| h.to_str().ok())
            .map(|s| CsrfToken(s.to_string()))
            .ok_or_else(|| AppError::BadRequest("X-CSRF-Token header required".to_string()))
    }
}

/// Generate new CSRF token.
pub fn generate_csrf_token() -> String {
    Uuid::new_v4().to_string()
}

/// Verify CSRF token from session per D-110.
pub async fn verify_csrf_token(
    cache: &redis::aio::ConnectionManager,
    session_token: &str,
    csrf_token: &str,
) -> AppResult<()> {
    // TODO: Verify token matches session's stored token
    // TODO: Rotate on next session update
    Ok(())
}
