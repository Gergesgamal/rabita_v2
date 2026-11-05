use crate::state::CacheConnection;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::auth::SessionClaim;
use crate::cart::CartManager;
use crate::error::AppError;
use crate::state::AppState;

use super::manager::CheckoutManager;
use super::models::*;

pub fn checkout_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/stores/:store_id/checkout/validate",
            post(validate_checkout),
        )
        .route(
            "/stores/:store_id/checkout/summary",
            post(get_checkout_summary),
        )
}

async fn validate_checkout(
    State(state): State<AppState>,
    SessionClaim {
        consumer_id,
        account_id,
        ..
    }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CheckoutRequest>,
) -> Result<Json<CheckoutCartValidation>, AppError> {
    let consumer_key = consumer_id
        .unwrap_or_else(|| account_id.to_string());

    let mut cache = state
        .cache
        .get_connection()
        .await
        .map_err(|e| AppError::new(
            crate::error::ErrorCode::CacheError,
            &format!("Connection error: {}", e),
        ))?;

    let cart = CartManager::get_cart(&mut cache, store_id, &consumer_key).await?;

    let validation = CheckoutManager::validate_cart(
        &state.db,
        store_id,
        &cart,
        &req.delivery_governorate,
        &req.delivery_county,
        &req.delivery_district,
        req.branch_pickup_id,
    )
    .await?;

    Ok(Json(validation))
}

async fn get_checkout_summary(
    State(state): State<AppState>,
    SessionClaim {
        consumer_id,
        account_id,
        ..
    }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CheckoutRequest>,
) -> Result<Json<CheckoutSummary>, AppError> {
    let consumer_key = consumer_id
        .unwrap_or_else(|| account_id.to_string());

    let mut cache = state
        .cache
        .get_connection()
        .await
        .map_err(|e| AppError::new(
            crate::error::ErrorCode::CacheError,
            &format!("Connection error: {}", e),
        ))?;

    let cart = CartManager::get_cart(&mut cache, store_id, &consumer_key).await?;

    let summary = CheckoutManager::calculate_checkout(
        &state.db,
        store_id,
        &cart,
        &req.delivery_governorate,
        &req.delivery_county,
        &req.delivery_district,
        req.branch_pickup_id,
        req.coupon_code.as_deref(),
    )
    .await?;

    Ok(Json(summary))
}
