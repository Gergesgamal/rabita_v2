// backend/src/catalog/service.rs

//! Catalog service per D-063.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct CatalogManager;

impl CatalogManager {
    /// Create category per D-063.
    pub async fn create_category(
        db: &PgPool,
        store_id: Uuid,
        name: &str,
        slug: &str,
        description: Option<&str>,
    ) -> AppResult<Uuid> {
        let category_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.category (store_id, name, slug, description)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(store_id)
        .bind(name)
        .bind(slug)
        .bind(description)
        .fetch_one(db)
        .await?;

        Ok(category_id)
    }

    /// Create product per D-063.
    pub async fn create_product(
        db: &PgPool,
        store_id: Uuid,
        sku: Option<&str>,
        name: &str,
        description: Option<&str>,
        category_id: Option<Uuid>,
        base_price_egp: i64,
    ) -> AppResult<Uuid> {
        if base_price_egp < 0 {
            return Err(AppError::BadRequest("Price must be non-negative".to_string()));
        }

        let product_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.product (store_id, sku, name, description, category_id, base_price_egp)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
        )
        .bind(store_id)
        .bind(sku)
        .bind(name)
        .bind(description)
        .bind(category_id)
        .bind(base_price_egp)
        .fetch_one(db)
        .await?;

        Ok(product_id)
    }

    /// Create variant per D-063.
    pub async fn create_variant(
        db: &PgPool,
        product_id: Uuid,
        name: &str,
        sku: Option<&str>,
        price_override_egp: Option<i64>,
    ) -> AppResult<Uuid> {
        if let Some(price) = price_override_egp {
            if price < 0 {
                return Err(AppError::BadRequest("Price must be non-negative".to_string()));
            }
        }

        let variant_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.variant (product_id, name, sku, price_override_egp)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(product_id)
        .bind(name)
        .bind(sku)
        .bind(price_override_egp)
        .fetch_one(db)
        .await?;

        Ok(variant_id)
    }

    /// Get product by ID.
    pub async fn get_product(db: &PgPool, product_id: Uuid) -> AppResult<ProductRow> {
        sqlx::query_as::<_, ProductRow>(
            r#"
            SELECT id, store_id, sku, name, description, category_id, base_price_egp,
                   is_published, created_at, updated_at
            FROM app.product
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(product_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProductRow {
    pub id: Uuid,
    pub store_id: Uuid,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub base_price_egp: i64,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
