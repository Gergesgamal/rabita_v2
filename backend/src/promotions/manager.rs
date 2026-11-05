use crate::error::{AppError, ErrorCode};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct PromotionManager;

impl PromotionManager {
    /// Create coupon per D-053
    pub async fn create_coupon(
        db: &PgPool,
        store_id: Uuid,
        req: CreateCouponRequest,
    ) -> Result<Coupon, AppError> {
        // Validate that either amount or percent is specified
        if req.discount_amount_egp.is_none() && req.discount_percent_bp.is_none() {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "Either discount amount or percent must be specified",
            ));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.coupon (id, store_id, code, discount_amount_egp, discount_percent_bp, branch_id, is_active, max_uses, current_uses, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, true, $7, 0, $8, $9)
            "#,
        )
        .bind(&id)
        .bind(&store_id)
        .bind(&req.code)
        .bind(req.discount_amount_egp)
        .bind(req.discount_percent_bp)
        .bind(&req.branch_id)
        .bind(req.max_uses)
        .bind(req.expires_at.as_deref())
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Self::get_coupon(db, store_id, &req.code).await
    }

    /// Get coupon by code
    pub async fn get_coupon(
        db: &PgPool,
        store_id: Uuid,
        code: &str,
    ) -> Result<Coupon, AppError> {
        sqlx::query_as::<_, Coupon>(
            "SELECT * FROM app.coupon WHERE store_id = $1 AND code = $2 AND is_active = true",
        )
        .bind(&store_id)
        .bind(code)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Coupon not found or inactive"))
    }

    /// Apply coupon (increment usage)
    pub async fn apply_coupon(
        db: &PgPool,
        store_id: Uuid,
        code: &str,
    ) -> Result<i64, AppError> {
        let coupon = Self::get_coupon(db, store_id, code).await?;

        // Check max uses
        if let Some(max) = coupon.max_uses {
            if coupon.current_uses >= max {
                return Err(AppError::new(
                    ErrorCode::InvalidState,
                    "Coupon usage limit exceeded",
                ));
            }
        }

        // Check expiry
        if let Some(expires) = coupon.expires_at {
            if expires < Utc::now() {
                return Err(AppError::new(
                    ErrorCode::InvalidState,
                    "Coupon has expired",
                ));
            }
        }

        // Increment usage
        sqlx::query(
            "UPDATE app.coupon SET current_uses = current_uses + 1 WHERE id = $1",
        )
        .bind(&coupon.id)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Return discount in piastres
        Ok(coupon.discount_amount_egp.unwrap_or(0))
    }

    /// Create bundle per D-053
    pub async fn create_bundle(
        db: &PgPool,
        store_id: Uuid,
        req: CreateBundleRequest,
    ) -> Result<Bundle, AppError> {
        if req.product_ids.is_empty() {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "Bundle must contain at least one product",
            ));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.bundle (id, store_id, name, description, discount_amount_egp, discount_percent_bp, allow_partial_removal, max_partial_applications, is_active, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true, $9)
            "#,
        )
        .bind(&id)
        .bind(&store_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.discount_amount_egp)
        .bind(req.discount_percent_bp)
        .bind(req.allow_partial_removal.unwrap_or(false))
        .bind(req.max_partial_applications.unwrap_or(1))
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Add bundle items
        for product_id in req.product_ids {
            sqlx::query(
                "INSERT INTO app.bundle_item (id, bundle_id, product_id, required_quantity) VALUES ($1, $2, $3, 1)",
            )
            .bind(Uuid::new_v4())
            .bind(&id)
            .bind(&product_id)
            .execute(db)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        }

        Self::get_bundle(db, store_id, id).await
    }

    /// Get bundle
    pub async fn get_bundle(
        db: &PgPool,
        store_id: Uuid,
        bundle_id: Uuid,
    ) -> Result<Bundle, AppError> {
        sqlx::query_as::<_, Bundle>(
            "SELECT * FROM app.bundle WHERE id = $1 AND store_id = $2 AND is_active = true",
        )
        .bind(&bundle_id)
        .bind(&store_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Bundle not found"))
    }
}
