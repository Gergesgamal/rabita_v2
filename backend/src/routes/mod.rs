// backend/src/routes/mod.rs

//! HTTP route definitions and router assembly.

pub mod health;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use crate::state::AppState;

/// Build the root router with all route modules.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .fallback(not_found)
}

/// 404 handler for unmatched routes.
async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Route not found. Most features are not yet implemented (Wave 1 foundation).",
    )
}
