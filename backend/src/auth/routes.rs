use axum::{http::StatusCode, routing::post, Router};

use crate::state::AppState;

// Placeholder until the real login/logout handlers are restored from the original source.
// Returns 501 so it cannot be mistaken for a working endpoint.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(not_implemented))
        .route("/logout", post(not_implemented))
}

async fn not_implemented() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
