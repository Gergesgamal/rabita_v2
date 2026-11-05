// Bearer-token extractor. Handlers take `SessionClaim` as an argument to require a session.
// Uses only axum 0.7 APIs; the original `axum::headers::TypedHeader` does not exist in axum 0.7.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};

use crate::{
    auth::session::{SessionClaim, SessionManager},
    error::AppError,
    state::AppState,
};

#[async_trait]
impl FromRequestParts<AppState> for SessionClaim {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .filter(|t| !t.is_empty())
            .ok_or(AppError::Unauthorized)?;

        SessionManager::verify_session(&state.cache, token).await
    }
}
