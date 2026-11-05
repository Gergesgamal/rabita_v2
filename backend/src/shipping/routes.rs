use crate::error::ErrorCode;
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

use super::manager::ShippingManager;
use super::models::*;

pub fn shipping_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/stores/:store_id/shipping-rules",
            post(create_rule).get(list_rules),
        )
        .route(
            "/stores/:store_id/shipping-rules/:rule_id",
            get(get_rule),
        )
}

async fn create_rule(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateShippingRuleRequest>,
) -> Result<(StatusCode, Json<ShippingRule>), AppError> {
    let rule = ShippingManager::create_rule(&state.db, account_id, store_id, req).await?;
    Ok((StatusCode::CREATED, Json(rule)))
}

async fn get_rule(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, rule_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ShippingRule>, AppError> {
    let rule = ShippingManager::get_rule(&state.db, account_id, store_id, rule_id).await?;
    Ok(Json(rule))
}

async fn list_rules(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<ShippingRule>>, AppError> {
    // Verify store ownership
    let _: (Uuid,) = sqlx::query_as(
        "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
    )
    .bind(&store_id)
    .bind(&account_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?
    .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

    let rules: Vec<ShippingRule> =
        sqlx::query_as("SELECT * FROM app.shipping_rule WHERE store_id = $1 ORDER BY created_at")
            .bind(&store_id)
            .fetch_all(&state.db)
            .await
            .map_err(|e| AppError::database_error(&e))?;

    Ok(Json(rules))
}
