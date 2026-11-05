use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::auth::SessionClaim;
use crate::error::AppError;
use crate::state::AppState;

use super::manager::ConsumerManager;
use super::models::*;

pub fn consumer_routes() -> Router<AppState> {
    Router::new()
        .route("/stores/:store_id/consumer/profile", get(get_profile).put(update_profile))
        .route("/stores/:store_id/consumer/addresses", post(add_address).get(list_addresses))
        .route("/stores/:store_id/consumer/consent", put(update_consent))
        .route("/stores/:store_id/consumer/delete", post(request_deletion))
}

async fn get_profile(
    State(state): State<AppState>,
    SessionClaim { consumer_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<ConsumerProfileResponse>, AppError> {
    let consumer = ConsumerManager::get_consumer(&state.db, store_id, consumer_id).await?;

    let addresses: Vec<ConsumerAddress> = sqlx::query_as(
        "SELECT * FROM app.consumer_address WHERE consumer_id = $1",
    )
    .bind(&consumer_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    let consent: ConsumerConsent = sqlx::query_as(
        "SELECT * FROM app.consumer_consent WHERE consumer_id = $1",
    )
    .bind(&consumer_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    Ok(Json(ConsumerProfileResponse {
        id: consumer.id.to_string(),
        email: consumer.email,
        name: consumer.name,
        phone: consumer.phone,
        addresses,
        consent,
    }))
}

async fn update_profile(
    State(state): State<AppState>,
    SessionClaim { consumer_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<ConsumerAccount>, AppError> {
    let consumer = ConsumerManager::update_profile(&state.db, store_id, consumer_id, req).await?;
    Ok(Json(consumer))
}

async fn add_address(
    State(state): State<AppState>,
    SessionClaim { consumer_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<AddAddressRequest>,
) -> Result<(StatusCode, Json<ConsumerAddress>), AppError> {
    let address = ConsumerManager::add_address(&state.db, store_id, consumer_id, req).await?;
    Ok((StatusCode::CREATED, Json(address)))
}

async fn list_addresses(
    State(state): State<AppState>,
    SessionClaim { consumer_id, .. }: SessionClaim,
) -> Result<Json<Vec<ConsumerAddress>>, AppError> {
    let addresses: Vec<ConsumerAddress> = sqlx::query_as(
        "SELECT * FROM app.consumer_address WHERE consumer_id = $1",
    )
    .bind(&consumer_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    Ok(Json(addresses))
}

async fn update_consent(
    State(state): State<AppState>,
    SessionClaim { consumer_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<UpdateConsentRequest>,
) -> Result<Json<ConsumerConsent>, AppError> {
    let consent = ConsumerManager::update_consent(&state.db, store_id, consumer_id, req).await?;
    Ok(Json(consent))
}

async fn request_deletion(
    State(state): State<AppState>,
    SessionClaim { consumer_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    ConsumerManager::request_deletion(&state.db, store_id, consumer_id).await?;
    Ok(StatusCode::OK)
}
