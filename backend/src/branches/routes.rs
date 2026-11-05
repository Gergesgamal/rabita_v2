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

use super::manager::BranchManager;
use super::models::*;

pub fn branch_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/stores/:store_id/branches",
            post(create_branch).get(list_branches),
        )
        .route(
            "/stores/:store_id/branches/:branch_id",
            get(get_branch).put(update_branch),
        )
        .route(
            "/stores/:store_id/branches/:branch_id/working-hours",
            put(set_working_hours),
        )
        .route(
            "/stores/:store_id/branches/:branch_id/stock/adjust",
            post(adjust_stock),
        )
}

async fn create_branch(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateBranchRequest>,
) -> Result<(StatusCode, Json<Branch>), AppError> {
    let branch = BranchManager::create_branch(&state.db, account_id, store_id, req).await?;
    Ok((StatusCode::CREATED, Json(branch)))
}

async fn list_branches(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<Branch>>, AppError> {
    let branches = BranchManager::list_branches(&state.db, account_id, store_id).await?;
    Ok(Json(branches))
}

async fn get_branch(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, branch_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Branch>, AppError> {
    let branch = BranchManager::get_branch(&state.db, account_id, store_id, branch_id).await?;
    Ok(Json(branch))
}

async fn update_branch(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, branch_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateBranchRequest>,
) -> Result<Json<Branch>, AppError> {
    let branch =
        BranchManager::update_branch(&state.db, account_id, store_id, branch_id, req).await?;
    Ok(Json(branch))
}

async fn set_working_hours(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, branch_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<SetWorkingHoursRequest>,
) -> Result<Json<Vec<BranchWorkingHours>>, AppError> {
    let hours =
        BranchManager::set_working_hours(&state.db, account_id, store_id, branch_id, req).await?;
    Ok(Json(hours))
}

async fn adjust_stock(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, branch_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<AdjustStockRequest>,
) -> Result<Json<BranchProduct>, AppError> {
    let bp =
        BranchManager::adjust_stock(&state.db, account_id, store_id, branch_id, req).await?;
    Ok(Json(bp))
}
