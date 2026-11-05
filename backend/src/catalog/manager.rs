use crate::error::{AppError, ErrorCode};
use crate::audit::AuditLogger;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct CatalogManager;

impl CatalogManager {
    /// Create category.
    pub async fn create_category(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        req: CreateCategoryRequest,
    ) -> Result<Category, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        let slug = slugify(&req.name);
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.category (id, store_id, name, slug, description, parent_id, is_published, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, false, $7, $7)
            "#,
        )
        .bind(&id)
        .bind(&store_id)
        .bind(&req.name)
        .bind(&slug)
        .bind(&req.description)
        .bind(&req.parent_id)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            "category.create",
            Some(id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_category(pool, account_id, store_id, id).await
    }

    /// Create product.
    pub async fn create_product(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        req: CreateProductRequest,
    ) -> Result<Product, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        if req.base_price_egp < 0 {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "Price cannot be negative",
            ));
        }

        let slug = slugify(&req.name);
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.product (id, store_id, name, slug, description, sku, base_price_egp, is_published, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, false, $8, $8)
            "#,
        )
        .bind(&id)
        .bind(&store_id)
        .bind(&req.name)
        .bind(&slug)
        .bind(&req.description)
        .bind(&req.sku)
        .bind(req.base_price_egp)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            "product.create",
            Some(id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_product(pool, account_id, store_id, id).await
    }

    /// Get product.
    pub async fn get_product(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        product_id: Uuid,
    ) -> Result<Product, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        sqlx::query_as::<_, Product>(
            "SELECT * FROM app.product WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL",
        )
        .bind(&product_id)
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Product not found"))
    }

    /// List products for store.
    pub async fn list_products(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
    ) -> Result<Vec<Product>, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        sqlx::query_as::<_, Product>(
            "SELECT * FROM app.product WHERE store_id = $1 AND deleted_at IS NULL ORDER BY name",
        )
        .bind(&store_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Create variant.
    pub async fn create_variant(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        product_id: Uuid,
        req: CreateVariantRequest,
    ) -> Result<Variant, AppError> {
        // Verify product ownership
        let _product = Self::get_product(pool, account_id, store_id, product_id).await?;

        if let Some(price) = req.price_override_egp {
            if price < 0 {
                return Err(AppError::new(
                    ErrorCode::InvalidInput,
                    "Price cannot be negative",
                ));
            }
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.variant (id, product_id, name, sku, price_override_egp, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $6)
            "#,
        )
        .bind(&id)
        .bind(&product_id)
        .bind(&req.name)
        .bind(&req.sku)
        .bind(req.price_override_egp)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            "variant.create",
            Some(id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_variant(pool, account_id, store_id, product_id, id).await
    }

    /// Get variant.
    pub async fn get_variant(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        product_id: Uuid,
        variant_id: Uuid,
    ) -> Result<Variant, AppError> {
        // Verify product ownership
        let _product = Self::get_product(pool, account_id, store_id, product_id).await?;

        sqlx::query_as::<_, Variant>(
            "SELECT * FROM app.variant WHERE id = $1 AND product_id = $2",
        )
        .bind(&variant_id)
        .bind(&product_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Variant not found"))
    }

    /// Publish product.
    pub async fn publish_product(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        product_id: Uuid,
        publish: bool,
    ) -> Result<Product, AppError> {
        let product = Self::get_product(pool, account_id, store_id, product_id).await?;
        let now = Utc::now();

        sqlx::query(
            "UPDATE app.product SET is_published = $1, updated_at = $2 WHERE id = $3",
        )
        .bind(publish)
        .bind(&now)
        .bind(&product_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            if publish { "product.publish" } else { "product.unpublish" },
            Some(product_id),
            serde_json::json!({}),
            None,
        )
        .await?;

        Self::get_product(pool, account_id, store_id, product_id).await
    }

    /// Add product image.
    pub async fn add_product_image(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        product_id: Uuid,
        req: AddProductImageRequest,
    ) -> Result<ProductImage, AppError> {
        // Verify product ownership
        let _product = Self::get_product(pool, account_id, store_id, product_id).await?;

        // Verify media ownership (belongs to store)
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.media WHERE id = $1 AND store_id = $2",
        )
        .bind(&req.media_id)
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Media not found"))?;

        let id = Uuid::new_v4();
        let now = Utc::now();

        // Get next sort order
        let (sort_order,): (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM app.product_image WHERE product_id = $1",
        )
        .bind(&product_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        sqlx::query(
            "INSERT INTO app.product_image (id, product_id, media_id, sort_order, created_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&id)
        .bind(&product_id)
        .bind(&req.media_id)
        .bind(sort_order)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(ProductImage {
            id,
            product_id,
            media_id: req.media_id,
            sort_order,
            created_at: now,
        })
    }
}

/// Simple slug generation: lowercase, remove special chars, replace spaces with hyphens.
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                continue;
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Get category
impl CatalogManager {
    pub async fn get_category(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        category_id: Uuid,
    ) -> Result<Category, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        sqlx::query_as::<_, Category>(
            "SELECT * FROM app.category WHERE id = $1 AND store_id = $2",
        )
        .bind(&category_id)
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Category not found"))
    }
}
