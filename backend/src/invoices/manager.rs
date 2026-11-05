use crate::error::AppError;
use uuid::Uuid;
use sqlx::PgPool;

use super::models::Invoice;

pub struct InvoiceManager;

impl InvoiceManager {
    /// Generate invoice PDF per D-122
    /// Arabic-first template per D-127
    pub async fn generate_invoice_pdf(
        db: &PgPool,
        store_id: Uuid,
        order_id: Uuid,
    ) -> Result<Vec<u8>, AppError> {
        // Fetch order and store data
        let order: (String, String, i64) = sqlx::query_as(
            "SELECT public_order_number, consumer_email, (order_snapshot ->> 'total_egp')::bigint FROM app.order WHERE id = $1 AND store_id = $2",
        )
        .bind(&order_id)
        .bind(&store_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let store: (String, Option<String>) = sqlx::query_as(
            "SELECT name, logo_media_id FROM app.store WHERE id = $1",
        )
        .bind(&store_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // TODO: Use a PDF library (printpdf, tinytemplate) to generate invoice
        // For now, return stub
        let pdf_data = vec![0u8; 1024];

        Ok(pdf_data)
    }

    /// Get invoice number per D-122 (configurable format)
    pub async fn generate_invoice_number(
        db: &PgPool,
        store_id: Uuid,
    ) -> Result<String, AppError> {
        // Check counter type
        let counter_type: (String,) = sqlx::query_as(
            "SELECT counter_type FROM app.invoice_counter WHERE store_id = $1",
        )
        .bind(&store_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .unwrap_or(("sequential_per_store".to_string(),));

        let store: (String,) = sqlx::query_as("SELECT slug FROM app.store WHERE id = $1")
            .bind(&store_id)
            .fetch_one(db)
            .await
            .map_err(|e| AppError::database_error(&e))?;

        // Get next number
        let next_num: (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(CAST(REGEXP_SUBSTR(invoice_number, '[0-9]+$') AS INTEGER)), 0) + 1 FROM app.invoice WHERE store_id = $1",
        )
        .bind(&store_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(format!("{}-{:06}", store.0, next_num.0))
    }
}
