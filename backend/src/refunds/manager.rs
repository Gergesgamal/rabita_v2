use crate::error::{AppError, ErrorCode};
use crate::audit::AuditLogger;
use crate::orders::OrderManager;
use crate::orders::stock_reserve::StockReserveManager;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct RefundManager;

impl RefundManager {
    /// Request refund per D-068
    pub async fn request_refund(
        db: &PgPool,
        store_id: Uuid,
        order_id: Uuid,
        consumer_id: Option<Uuid>,
        req: CreateRefundRequestReq,
    ) -> Result<RefundRequest, AppError> {
        // Verify order exists and belongs to store
        let order: (i64,) = sqlx::query_as(
            "SELECT refund_amount_egp FROM app.order WHERE id = $1 AND store_id = $2 AND order_status IN ('delivered')",
        )
        .bind(&order_id)
        .bind(&store_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(
            ErrorCode::NotFound,
            "Order not found or not eligible for refund",
        ))?;

        let refund_amount_egp = order.0;

        // Create refund request
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.refund_request (id, order_id, consumer_id, reason, refund_amount_egp, state, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, 'pending', $6, $6)
            "#,
        )
        .bind(&id)
        .bind(&order_id)
        .bind(&consumer_id)
        .bind(&req.reason)
        .bind(refund_amount_egp)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            db,
            consumer_id.unwrap_or(Uuid::nil()),
            "refund.request",
            Some(id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_refund(db, store_id, id).await
    }

    /// Approve or reject refund per D-068
    pub async fn merchant_decide_refund(
        db: &PgPool,
        store_id: Uuid,
        account_id: Uuid,
        refund_id: Uuid,
        approved: bool,
        reason: Option<&str>,
    ) -> Result<RefundRequest, AppError> {
        let mut tx = db.begin().await.map_err(|e| AppError::database_error(&e))?;

        // Get refund
        let refund: (Uuid, String) = sqlx::query_as(
            "SELECT order_id, state FROM app.refund_request WHERE id = $1",
        )
        .bind(&refund_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if refund.1 != "pending" {
            return Err(AppError::new(
                ErrorCode::InvalidState,
                "Refund not in pending state",
            ));
        }

        let new_state = if approved { "approved" } else { "rejected" };
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE app.refund_request
            SET state = $1, merchant_decision = $2, merchant_decision_reason = $3, updated_at = $4
            WHERE id = $5
            "#,
        )
        .bind(new_state)
        .bind(if approved { "approved" } else { "rejected" })
        .bind(reason)
        .bind(&now)
        .bind(&refund_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // If approved, mark order as refunded per D-068
        if approved {
            sqlx::query(
                "UPDATE app.order SET payment_status = 'refunded', updated_at = $1 WHERE id = $2 AND store_id = $3",
            )
            .bind(&now)
            .bind(&refund.0)
            .bind(&store_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            // Release stock per D-051
            StockReserveManager::release_stock(&mut *tx, refund.0)
                .await?;
        }

        AuditLogger::log_action(
            &mut *tx,
            account_id,
            if approved { "refund.approve" } else { "refund.reject" },
            Some(refund_id),
            serde_json::json!({ "reason": reason }),
            None,
        )
        .await?;

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        Self::get_refund(db, store_id, refund_id).await
    }

    /// Get refund
    pub async fn get_refund(
        db: &PgPool,
        store_id: Uuid,
        refund_id: Uuid,
    ) -> Result<RefundRequest, AppError> {
        sqlx::query_as::<_, RefundRequest>(
            "SELECT r.* FROM app.refund_request r JOIN app.order o ON r.order_id = o.id WHERE r.id = $1 AND o.store_id = $2",
        )
        .bind(&refund_id)
        .bind(&store_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Refund not found"))
    }

    /// List refunds for store
    pub async fn list_refunds(
        db: &PgPool,
        store_id: Uuid,
    ) -> Result<Vec<RefundRequest>, AppError> {
        sqlx::query_as::<_, RefundRequest>(
            "SELECT r.* FROM app.refund_request r JOIN app.order o ON r.order_id = o.id WHERE o.store_id = $1 ORDER BY r.created_at DESC LIMIT 100",
        )
        .bind(&store_id)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::database_error(&e))
    }
}
