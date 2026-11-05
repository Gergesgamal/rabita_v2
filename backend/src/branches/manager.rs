use crate::error::{AppError, ErrorCode};
use crate::audit::AuditLogger;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct BranchManager;

impl BranchManager {
    /// Create a branch for a store.
    pub async fn create_branch(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        req: CreateBranchRequest,
    ) -> Result<Branch, AppError> {
        // Verify store ownership
        let _store: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        let branch_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.branch (id, store_id, name, address, governorate, county, district, phone, is_published, allow_pickup, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, false, $9, $10, $10)
            "#,
        )
        .bind(&branch_id)
        .bind(&store_id)
        .bind(&req.name)
        .bind(&req.address)
        .bind(&req.governorate)
        .bind(&req.county)
        .bind(&req.district)
        .bind(&req.phone)
        .bind(req.allow_pickup.unwrap_or(false))
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Initialize working hours (all closed by default)
        for day in 0..7 {
            sqlx::query(
                "INSERT INTO app.branch_working_hours (id, branch_id, day_of_week, is_closed, open_time, close_time) VALUES ($1, $2, $3, true, NULL, NULL)"
            )
            .bind(Uuid::new_v4())
            .bind(&branch_id)
            .bind(day)
            .execute(pool)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        }

        AuditLogger::log_action(
            pool,
            account_id,
            "branch.create",
            Some(branch_id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_branch(pool, account_id, store_id, branch_id).await
    }

    /// Get branch (validate store ownership).
    pub async fn get_branch(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        branch_id: Uuid,
    ) -> Result<Branch, AppError> {
        // Verify store ownership first
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        sqlx::query_as::<_, Branch>(
            "SELECT * FROM app.branch WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL",
        )
        .bind(&branch_id)
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Branch not found"))
    }

    /// List branches for store.
    pub async fn list_branches(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
    ) -> Result<Vec<Branch>, AppError> {
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

        sqlx::query_as::<_, Branch>(
            "SELECT * FROM app.branch WHERE store_id = $1 AND deleted_at IS NULL ORDER BY name",
        )
        .bind(&store_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Update branch.
    pub async fn update_branch(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        branch_id: Uuid,
        req: UpdateBranchRequest,
    ) -> Result<Branch, AppError> {
        let _branch = Self::get_branch(pool, account_id, store_id, branch_id).await?;
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE app.branch
            SET name = COALESCE($1, name),
                address = COALESCE($2, address),
                governorate = COALESCE($3, governorate),
                county = COALESCE($4, county),
                district = COALESCE($5, district),
                phone = COALESCE($6, phone),
                allow_pickup = COALESCE($7, allow_pickup),
                updated_at = $8
            WHERE id = $9
            "#,
        )
        .bind(&req.name)
        .bind(&req.address)
        .bind(&req.governorate)
        .bind(&req.county)
        .bind(&req.district)
        .bind(&req.phone)
        .bind(&req.allow_pickup)
        .bind(&now)
        .bind(&branch_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            "branch.update",
            Some(branch_id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_branch(pool, account_id, store_id, branch_id).await
    }

    /// Set working hours for branch (replace existing).
    pub async fn set_working_hours(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        branch_id: Uuid,
        req: SetWorkingHoursRequest,
    ) -> Result<Vec<BranchWorkingHours>, AppError> {
        let _branch = Self::get_branch(pool, account_id, store_id, branch_id).await?;

        let mut tx = pool.begin().await.map_err(|e| AppError::database_error(&e))?;

        // Delete existing hours
        sqlx::query("DELETE FROM app.branch_working_hours WHERE branch_id = $1")
            .bind(&branch_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;

        // Insert new hours
        for hour in &req.hours {
            sqlx::query(
                "INSERT INTO app.branch_working_hours (id, branch_id, day_of_week, is_closed, open_time, close_time) VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(Uuid::new_v4())
            .bind(&branch_id)
            .bind(hour.day_of_week)
            .bind(hour.is_closed)
            .bind(&hour.open_time)
            .bind(&hour.close_time)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        }

        AuditLogger::log_action(
            &mut *tx,
            account_id,
            "branch.set_working_hours",
            Some(branch_id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        sqlx::query_as::<_, BranchWorkingHours>(
            "SELECT * FROM app.branch_working_hours WHERE branch_id = $1 ORDER BY day_of_week",
        )
        .bind(&branch_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Adjust stock (inventory.adjust permission).
    pub async fn adjust_stock(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        branch_id: Uuid,
        req: AdjustStockRequest,
    ) -> Result<BranchProduct, AppError> {
        // Verify store and branch ownership
        let _branch = Self::get_branch(pool, account_id, store_id, branch_id).await?;

        // Verify product belongs to store
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.product WHERE id = $1 AND store_id = $2",
        )
        .bind(&req.product_id)
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Product not found"))?;

        let mut tx = pool.begin().await.map_err(|e| AppError::database_error(&e))?;

        // Get or create branch_product record
        let bp_id: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM app.branch_product WHERE branch_id = $1 AND product_id = $2",
        )
        .bind(&branch_id)
        .bind(&req.product_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let bp_id = if let Some((id,)) = bp_id {
            id
        } else {
            let new_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO app.branch_product (id, branch_id, product_id, stock_qty, is_published) VALUES ($1, $2, $3, 0, false)"
            )
            .bind(&new_id)
            .bind(&branch_id)
            .bind(&req.product_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;
            new_id
        };

        // Update stock
        let now = Utc::now();
        sqlx::query(
            "UPDATE app.branch_product SET stock_qty = stock_qty + $1, updated_at = $2 WHERE id = $3",
        )
        .bind(req.quantity_delta)
        .bind(&now)
        .bind(&bp_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Audit
        AuditLogger::log_action(
            &mut *tx,
            account_id,
            "inventory.adjust",
            Some(bp_id),
            serde_json::json!({ "delta": req.quantity_delta, "reason": req.reason }),
            None,
        )
        .await?;

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        sqlx::query_as::<_, BranchProduct>(
            "SELECT * FROM app.branch_product WHERE id = $1",
        )
        .bind(&bp_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }
}
