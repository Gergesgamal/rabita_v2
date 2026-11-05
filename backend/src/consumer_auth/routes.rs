// backend/src/consumer_auth/routes.rs

//! Consumer authentication routes.

use crate::{
    consumer_auth::{
        login::ConsumerLoginManager,
        models::{ConsumerSignupRequest, ConsumerSignupResponse, ConsumerLoginRequest, ConsumerLoginResponse},
        signup::ConsumerSignupManager,
    },
    error::AppResult,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/signup", axum::routing::post(signup))
        .route("/verify", axum::routing::post(verify))
        .route("/:store_slug/login", axum::routing::post(login))
}

/// POST /api/v1/consumer/signup
async fn signup(
    State(state): State<AppState>,
    Json(input): Json<ConsumerSignupRequest>,
) -> AppResult<(StatusCode, Json<ConsumerSignupResponse>)> {
    // TODO: Extract store_id from context or request
    let store_id = uuid::Uuid::nil(); // Placeholder

    let (_consumer_id, verification_method) = ConsumerSignupManager::signup(
        &state.db,
        store_id,
        &input.email,
        &input.password,
        &input.name,
        input.phone_number.as_deref(),
    )
    .await?;

    // Send verification based on method
    ConsumerSignupManager::send_email_verification(store_id, uuid::Uuid::nil(), &input.email)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ConsumerSignupResponse {
            email_verified: false,
            verification_sent_via: verification_method,
        }),
    ))
}

/// POST /api/v1/consumer/verify
async fn verify(
    State(state): State<AppState>,
    axum::Json(body): axum::Json<serde_json::Value>,
) -> AppResult<StatusCode> {
    // TODO: Parse verification_method and token
    Ok(StatusCode::OK)
}

/// POST /api/v1/consumer/:store_slug/login
async fn login(
    State(state): State<AppState>,
    Path(_store_slug): Path<String>,
    Json(input): Json<ConsumerLoginRequest>,
) -> AppResult<(StatusCode, Json<ConsumerLoginResponse>)> {
    // TODO: Resolve store_slug to store_id
    let store_id = uuid::Uuid::nil(); // Placeholder

    let consumer_id = ConsumerLoginManager::login(&state.db, store_id, &input.email, &input.password).await?;

    let session_token = ConsumerLoginManager::issue_session(&state.cache, store_id, consumer_id).await?;

    Ok((
        StatusCode::OK,
        Json(ConsumerLoginResponse { session_token }),
    ))
}
