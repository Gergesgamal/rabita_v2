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

use super::manager::CatalogManager;
use super::models::*;

pub fn catalog_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/stores/:store_id/categories",
            post(create_category).get(list_categories),
        )
        .route(
            "/stores/:store_id/categories/:category_id",
            get(get_category).put(update_category),
        )
        .route(
            "/stores/:store_id/products",
            post(create_product).get(list_products),
        )
        .route(
            "/stores/:store_id/products/:product_id",
            get(get_product).put(update_product),
        )
        .route(
            "/stores/:store_id/products/:product_id/publish",
            post(publish_product),
        )
        .route(
            "/stores/:store_id/products/:product_id/variants",
            post(create_variant).get(list_variants),
        )
        .route(
            "/stores/:store_id/products/:product_id/images",
            post(add_product_image),
        )
}

async fn create_category(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<Category>), AppError> {
    let category = CatalogManager::create_category(&state.db, account_id, store_id, req).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

async fn get_category(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, category_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Category>, AppError> {
    let category =
        CatalogManager::get_category(&state.db, account_id, store_id, category_id).await?;
    Ok(Json(category))
}

async fn list_categories(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories: Vec<Category> = sqlx::query_as(
        "SELECT * FROM app.category WHERE store_id = $1 ORDER BY name",
    )
    .bind(&store_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    Ok(Json(categories))
}

async fn update_category(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, category_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    // Verify ownership first
    let _category =
        CatalogManager::get_category(&state.db, account_id, store_id, category_id).await?;

    sqlx::query(
        "UPDATE app.category SET name = COALESCE($1, name), description = COALESCE($2, description) WHERE id = $3",
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&category_id)
    .execute(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    CatalogManager::get_category(&state.db, account_id, store_id, category_id).await
}

async fn create_product(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    let product = CatalogManager::create_product(&state.db, account_id, store_id, req).await?;
    Ok((StatusCode::CREATED, Json(product)))
}

async fn get_product(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, product_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Product>, AppError> {
    let product = CatalogManager::get_product(&state.db, account_id, store_id, product_id).await?;
    Ok(Json(product))
}

async fn list_products(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = CatalogManager::list_products(&state.db, account_id, store_id).await?;
    Ok(Json(products))
}

async fn update_product(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, product_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<Json<Product>, AppError> {
    let _product = CatalogManager::get_product(&state.db, account_id, store_id, product_id).await?;

    sqlx::query(
        "UPDATE app.product SET name = COALESCE($1, name), description = COALESCE($2, description), sku = COALESCE($3, sku), base_price_egp = COALESCE($4, base_price_egp) WHERE id = $5",
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.sku)
    .bind(req.base_price_egp)
    .bind(&product_id)
    .execute(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    CatalogManager::get_product(&state.db, account_id, store_id, product_id).await
}

async fn publish_product(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, product_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<PublishProductRequest>,
) -> Result<Json<Product>, AppError> {
    let product =
        CatalogManager::publish_product(&state.db, account_id, store_id, product_id, req.publish)
            .await?;
    Ok(Json(product))
}

async fn create_variant(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, product_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<CreateVariantRequest>,
) -> Result<(StatusCode, Json<Variant>), AppError> {
    let variant =
        CatalogManager::create_variant(&state.db, account_id, store_id, product_id, req)
            .await?;
    Ok((StatusCode::CREATED, Json(variant)))
}

async fn list_variants(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, product_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Variant>>, AppError> {
    let _product = CatalogManager::get_product(&state.db, account_id, store_id, product_id).await?;

    let variants: Vec<Variant> =
        sqlx::query_as("SELECT * FROM app.variant WHERE product_id = $1 ORDER BY name")
            .bind(&product_id)
            .fetch_all(&state.db)
            .await
            .map_err(|e| AppError::database_error(&e))?;

    Ok(Json(variants))
}

async fn add_product_image(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, product_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<AddProductImageRequest>,
) -> Result<(StatusCode, Json<ProductImage>), AppError> {
    let image =
        CatalogManager::add_product_image(&state.db, account_id, store_id, product_id, req)
            .await?;
    Ok((StatusCode::CREATED, Json(image)))
}
