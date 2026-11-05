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

use super::manager::MediaManager;
use super::models::*;

pub fn media_routes() -> Router<AppState> {
    Router::new()
        .route("/stores/:store_id/media/upload-url", post(get_upload_url))
        .route("/stores/:store_id/media/:media_id", get(get_media))
        .route("/stores/:store_id/media/quota", get(get_quota))
}

async fn get_upload_url(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<GetUploadUrlRequest>,
) -> Result<(StatusCode, Json<UploadUrlResponse>), AppError> {
    let url = MediaManager::create_upload_url(&state.db, account_id, store_id, req).await?;
    Ok((StatusCode::CREATED, Json(url)))
}

async fn get_media(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, media_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Media>, AppError> {
    let media = MediaManager::get_media(&state.db, account_id, store_id, media_id).await?;
    Ok(Json(media))
}

async fn get_quota(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<QuotaResponse>, AppError> {
    let quota = MediaManager::get_quota(&state.db, account_id, store_id).await?;
    let percent_used = (quota.used_bytes as f64 / quota.quota_bytes as f64) * 100.0;

    Ok(Json(QuotaResponse {
        quota_bytes: quota.quota_bytes,
        used_bytes: quota.used_bytes,
        percent_used,
    }))
}
