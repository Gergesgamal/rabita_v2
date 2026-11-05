// backend/src/idempotency/middleware.rs

//! Idempotency middleware per D-130.

use crate::error::{AppError, AppResult};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};

/// Extractor for Idempotency-Key header.
#[derive(Debug, Clone)]
pub struct IdempotencyKey(pub String);

#[async_trait]
impl FromRequestParts<crate::state::AppState> for IdempotencyKey {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &crate::state::AppState,
    ) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("idempotency-key")
            .and_then(|h| h.to_str().ok())
            .map(|s| IdempotencyKey(s.to_string()))
            .ok_or_else(|| AppError::BadRequest("Idempotency-Key header required".to_string()))
    }
}
