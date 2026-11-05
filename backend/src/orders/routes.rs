use crate::state::CacheConnection;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    auth::SessionClaim,
    cart::CartManager,
    checkout::CheckoutManager,
    error::AppError,
    state::AppState,
};

use super::manager::OrderManager;
use super::models::*;

pub fn order_routes() -> Router<AppState> {
    Router::new()
        .route("/stores/:store_id/orders", post(create_order).get(list_orders))
        .route("/stores/:store_id/orders/:order_id", get(get_order))
        .route("/stores/:store_id/orders/:order_id/confirm", post(confirm_order))
        .route("/stores/:store_id/orders/:order_id/cancel", post(cancel_order))
}

async fn create_order(
    State(state): State<AppState>,
    SessionClaim {
        consumer_id,
        account_id,
        ..
    }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    let consumer_key = consumer_id
        .unwrap_or_else(|| account_id.to_string());

    let mut cache = state.cache.get_connection().await.map_err(|e| {
        AppError::new(
            crate::error::ErrorCode::CacheError,
            &format!("Connection error: {}", e),
        )
    })?;

    // Get cart
    let cart = CartManager::get_cart(&mut cache, store_id, &consumer_key).await?;

    // Calculate checkout
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

    // Create order with idempotency
    let email = req.email.clone();
    let total_egp = summary.total_egp;

    let order = OrderManager::create_order(
        &state.db,
        store_id,
        &email,
        "consumer",
        &consumer_key,
        "default-key", // TODO: from header
        req,
        cart,
        summary,
    )
    .await?;

    // Clear cart
    CartManager::clear_cart(&mut cache, store_id, &consumer_key)
        .await
        .ok();

    Ok((
        StatusCode::CREATED,
        Json(OrderResponse {
            id: order.id.to_string(),
            public_order_number: order.public_order_number,
            status: order.order_status,
            payment_status: order.payment_status,
            total_egp,
            created_at: order.created_at.to_rfc3339(),
        }),
    ))
}

async fn get_order(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, order_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Order>, AppError> {
    // TODO: Verify store ownership or consumer access
    let order = OrderManager::get_order(&state.db, store_id, order_id).await?;
    Ok(Json(order))
}

async fn list_orders(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<Order>>, AppError> {
    // TODO: Verify store ownership
    let orders = OrderManager::list_orders(&state.db, store_id).await?;
    Ok(Json(orders))
}

async fn confirm_order(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, order_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    // TODO: Verify store ownership
    OrderManager::confirm_order(&state.db, store_id, order_id).await?;
    Ok(StatusCode::OK)
}

async fn cancel_order(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, order_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    // TODO: Verify store ownership + cancellation permission
    OrderManager::cancel_order(&state.db, store_id, order_id).await?;
    Ok(StatusCode::OK)
}
