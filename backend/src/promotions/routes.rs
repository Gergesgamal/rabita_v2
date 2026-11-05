use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::auth::SessionClaim;
use crate::error::AppError;
use crate::state::AppState;

use super::manager::PromotionManager;
use super::models::*;

pub fn promotion_routes() -> Router<AppState> {
    Router::new()
        .route("/stores/:store_id/coupons", post(create_coupon))
        .route("/stores/:store_id/bundles", post(create_bundle).get(list_bundles))
}

async fn create_coupon(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateCouponRequest>,
) -> Result<(StatusCode, Json<Coupon>), AppError> {
    // TODO: Verify store ownership
    let coupon = PromotionManager::create_coupon(&state.db, store_id, req).await?;
    Ok((StatusCode::CREATED, Json(coupon)))
}

async fn create_bundle(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateBundleRequest>,
) -> Result<(StatusCode, Json<Bundle>), AppError> {
    // TODO: Verify store ownership
    let bundle = PromotionManager::create_bundle(&state.db, store_id, req).await?;
    Ok((StatusCode::CREATED, Json(bundle)))
}

async fn list_bundles(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<Bundle>>, AppError> {
    // TODO: Verify store ownership
    let bundles: Vec<Bundle> = sqlx::query_as(
        "SELECT * FROM app.bundle WHERE store_id = $1 AND is_active = true ORDER BY created_at DESC",
    )
    .bind(&store_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    Ok(Json(bundles))
}
