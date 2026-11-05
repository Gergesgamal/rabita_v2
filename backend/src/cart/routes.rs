use crate::state::CacheConnection;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::auth::SessionClaim;
use crate::error::AppError;
use crate::state::AppState;

use super::manager::CartManager;
use super::models::*;

pub fn cart_routes() -> Router<AppState> {
    Router::new()
        .route("/stores/:store_id/cart", get(get_cart).post(add_to_cart))
        .route("/stores/:store_id/cart/remove", delete(remove_from_cart))
        .route("/stores/:store_id/cart/clear", post(clear_cart))
}

async fn get_cart(
    State(state): State<AppState>,
    SessionClaim {
        account_id,
        consumer_id,
        ..
    }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Cart>, AppError> {
    // For guest: use session ID; for account: use consumer_id
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
    Ok(Json(cart))
}

async fn add_to_cart(
    State(state): State<AppState>,
    SessionClaim {
        consumer_id,
        account_id,
        ..
    }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<AddToCartRequest>,
) -> Result<Json<Cart>, AppError> {
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

    let cart = CartManager::add_to_cart(&mut cache, store_id, &consumer_key, req).await?;
    Ok(Json(cart))
}

async fn remove_from_cart(
    State(state): State<AppState>,
    SessionClaim {
        consumer_id,
        account_id,
        ..
    }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<RemoveFromCartRequest>,
) -> Result<StatusCode, AppError> {
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

    CartManager::remove_from_cart(&mut cache, store_id, &consumer_key, req).await?;
    Ok(StatusCode::OK)
}

async fn clear_cart(
    State(state): State<AppState>,
    SessionClaim {
        consumer_id,
        account_id,
        ..
    }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
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

    CartManager::clear_cart(&mut cache, store_id, &consumer_key).await?;
    Ok(StatusCode::OK)
}
