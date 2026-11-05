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

use super::manager::StoreManager;
use super::models::*;

pub fn store_routes() -> Router<AppState> {
    Router::new()
        .route("/stores", post(create_store).get(list_stores))
        .route("/stores/:store_id", get(get_store).put(update_store))
        .route("/stores/:store_id/settings", put(update_settings))
        .route("/stores/:store_id/publish", post(publish_store))
        .route("/stores/:store_id/suspend", post(suspend_store))
}

async fn create_store(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Json(req): Json<CreateStoreRequest>,
) -> Result<(StatusCode, Json<StoreResponse>), AppError> {
    let store = StoreManager::create_store(&state.db, account_id, req).await?;
    let settings = sqlx::query_as::<_, StoreSettings>(
        "SELECT * FROM app.store_settings WHERE store_id = $1",
    )
    .bind(&store.id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    Ok((
        StatusCode::CREATED,
        Json(StoreResponse {
            id: store.id.to_string(),
            slug: store.slug,
            name: store.name,
            description: store.description,
            is_published: store.is_published,
            is_suspended: store.is_suspended,
            contact_email: store.contact_email,
            contact_phone: store.contact_phone,
            created_at: store.created_at.to_rfc3339(),
            settings,
        }),
    ))
}

async fn list_stores(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
) -> Result<Json<Vec<Store>>, AppError> {
    let stores = StoreManager::list_stores(&state.db, account_id).await?;
    Ok(Json(stores))
}

async fn get_store(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Store>, AppError> {
    let store = StoreManager::get_store(&state.db, account_id, store_id).await?;
    Ok(Json(store))
}

async fn update_store(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<UpdateStoreRequest>,
) -> Result<Json<Store>, AppError> {
    let store = StoreManager::update_store(&state.db, account_id, store_id, req).await?;
    Ok(Json(store))
}

async fn update_settings(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<UpdateStoreSettingsRequest>,
) -> Result<Json<StoreSettings>, AppError> {
    let settings = StoreManager::update_settings(&state.db, account_id, store_id, req).await?;
    Ok(Json(settings))
}

async fn publish_store(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<PublishStoreRequest>,
) -> Result<Json<Store>, AppError> {
    let store = StoreManager::set_publication(&state.db, account_id, store_id, req.publish).await?;
    Ok(Json(store))
}

async fn suspend_store(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<SuspendStoreRequest>,
) -> Result<Json<Store>, AppError> {
    let store = StoreManager::suspend_store(&state.db, account_id, store_id, req).await?;
    Ok(Json(store))
}
