use crate::error::{AppError, ErrorCode};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::StockReserve;

pub struct StockReserveManager;

impl StockReserveManager {
    /// Create pending stock reserves at order creation per D-051
    pub async fn create_pending_reserves(
        db: &mut sqlx::PgConnection,
        order_id: Uuid,
        lines: Vec<(Uuid, Option<Uuid>, Uuid, i64)>, // (product_id, variant_id, branch_id, qty)
    ) -> Result<Vec<StockReserve>, AppError> {
        let now = Utc::now();
        let mut reserves = vec![];

        for (product_id, variant_id, branch_id, quantity) in lines {
            let reserve_id = Uuid::new_v4();

            sqlx::query(
                r#"
                INSERT INTO app.stock_reserve (id, order_id, product_id, variant_id, branch_id, quantity, state, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, 'pending', $7)
                "#,
            )
            .bind(&reserve_id)
            .bind(&order_id)
            .bind(&product_id)
            .bind(&variant_id)
            .bind(&branch_id)
            .bind(quantity)
            .bind(&now)
            .execute(&mut *db)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            reserves.push(StockReserve {
                id: reserve_id,
                order_id,
                product_id,
                variant_id,
                branch_id,
                quantity,
                state: "pending".to_string(),
                created_at: now,
            });
        }

        Ok(reserves)
    }

    /// Transition reserves to active and decrement stock atomically per D-051
    pub async fn activate_reserves_and_decrement_stock(
        db: &PgPool,
        order_id: Uuid,
    ) -> Result<(), AppError> {
        let mut tx = db.begin().await.map_err(|e| AppError::database_error(&e))?;

        // Get all pending reserves for this order
        let reserves: Vec<(Uuid, Uuid, Option<Uuid>, Uuid, i64)> = sqlx::query_as(
            "SELECT id, product_id, variant_id, branch_id, quantity FROM app.stock_reserve WHERE order_id = $1 AND state = 'pending'",
        )
        .bind(&order_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let now = Utc::now();

        for (reserve_id, product_id, variant_id, branch_id, quantity) in reserves {
            // Lock and check availability
            let current_stock: (i64,) = sqlx::query_as(
                "SELECT stock_qty FROM app.branch_product WHERE branch_id = $1 AND product_id = $2 FOR UPDATE",
            )
            .bind(&branch_id)
            .bind(&product_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            if current_stock.0 < quantity {
                return Err(AppError::new(
                    ErrorCode::InvalidState,
                    &format!("Insufficient stock for product {}", product_id),
                ));
            }

            // Decrement stock
            sqlx::query(
                "UPDATE app.branch_product SET stock_qty = stock_qty - $1 WHERE branch_id = $2 AND product_id = $3",
            )
            .bind(quantity)
            .bind(&branch_id)
            .bind(&product_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            // Transition reserve to active
            sqlx::query(
                "UPDATE app.stock_reserve SET state = 'active' WHERE id = $1",
            )
            .bind(&reserve_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        }

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        Ok(())
    }

    /// Release stock (cancellation, return, failure) per D-051
    pub async fn release_stock(
        db: &PgPool,
        order_id: Uuid,
    ) -> Result<(), AppError> {
        let mut tx = db.begin().await.map_err(|e| AppError::database_error(&e))?;

        // Get all active reserves
        let reserves: Vec<(Uuid, Uuid, Uuid, i64)> = sqlx::query_as(
            "SELECT id, branch_id, product_id, quantity FROM app.stock_reserve WHERE order_id = $1 AND state = 'active'",
        )
        .bind(&order_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        for (reserve_id, branch_id, product_id, quantity) in reserves {
            // Restore stock
            sqlx::query(
                "UPDATE app.branch_product SET stock_qty = stock_qty + $1 WHERE branch_id = $2 AND product_id = $3",
            )
            .bind(quantity)
            .bind(&branch_id)
            .bind(&product_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            // Transition to released
            sqlx::query(
                "UPDATE app.stock_reserve SET state = 'released' WHERE id = $1",
            )
            .bind(&reserve_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        }

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        Ok(())
    }
}
