// backend/src/routes/health.rs

//! Health check endpoints.

use crate::{
    db,
    state::AppState,
};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

/// GET /health — simple liveness check (no dependencies).
pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "rabita-backend"
        })),
    )
}

/// GET /ready — readiness check (dependencies required).
pub async fn ready(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let mut failing = Vec::new();

    // Check PostgreSQL
    if let Err(e) = db::check_postgres(&state.db).await {
        failing.push(format!("postgres: {}", e));
    }

    // Check Valkey
    if let Err(e) = db::check_valkey(&state.cache).await {
        failing.push(format!("valkey: {}", e));
    }

    if failing.is_empty() {
        (
            StatusCode::OK,
            Json(json!({
                "ready": true,
                "dependencies": {
                    "postgres": "ok",
                    "valkey": "ok"
                }
            })),
        )
            .into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "ready": false,
                "failing": failing
            })),
        )
            .into_response()
    }
}
