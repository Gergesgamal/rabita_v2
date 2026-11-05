use crate::checkout::CheckoutSummary;
use sha2::Digest;
use crate::error::{AppError, ErrorCode};
use crate::idempotency::IdempotencyStore;
use crate::checkout::CheckoutManager;
use crate::cart::{Cart, CartManager};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use serde_json::json;

use super::models::*;
use super::stock_reserve::StockReserveManager;

pub struct OrderManager;

impl OrderManager {
    /// Create order with idempotency per D-130 (principal-scoped key + payload hash)
    /// Returns the public order number
    pub async fn create_order(
        db: &PgPool,
        store_id: Uuid,
        consumer_email: &str,
        principal_type: &str,        // "consumer" or "guest_session"
        principal_id: &str,           // consumer UUID or session ID
        idempotency_key: &str,
        req: CreateOrderRequest,
        cart: Cart,
        checkout_summary: CheckoutSummary,
    ) -> Result<Order, AppError> {
        // Check idempotency (per D-130: (principal_type, principal_id, key) scope)
        let payload_hash = format!(
            "{}{}{}{}",
            store_id, consumer_email, req.delivery_address, checkout_summary.total_egp
        );
        let payload_hash = sha2::Sha256::digest(payload_hash.as_bytes())
            .to_vec();
        let payload_hash_hex = hex::encode(&payload_hash);

        let idempotency_result: Option<(Option<String>, Option<String>)> = sqlx::query_as(
            "SELECT response_status, response_body FROM app.idempotency_key WHERE principal_type = $1 AND principal_id = $2 AND key = $3",
        )
        .bind(principal_type)
        .bind(principal_id)
        .bind(idempotency_key)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if let Some((status, body)) = idempotency_result {
            if let Some(body_str) = body {
                // Return cached response
                let cached: serde_json::Value = serde_json::from_str(&body_str)
                    .map_err(|e| AppError::new(
                        ErrorCode::InternalError,
                        &format!("JSON parse: {}", e),
                    ))?;
                // TODO: Parse and return the cached order
                return Ok(Order {
                    id: Uuid::new_v4(),
                    store_id,
                    public_order_number: cached["public_order_number"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    consumer_email: consumer_email.to_string(),
                    consumer_phone: None,
                    verification_method: "email_only".to_string(),
                    order_snapshot: json!({}),
                    order_status: "pending".to_string(),
                    payment_status: "delayed".to_string(),
                    delivery_governorate: req.delivery_governorate.clone(),
                    delivery_county: req.delivery_county.clone(),
                    delivery_district: req.delivery_district.clone(),
                    delivery_address: req.delivery_address.clone(),
                    delivery_coords_lat: req.delivery_coords_lat,
                    delivery_coords_lng: req.delivery_coords_lng,
                    branch_pickup_id: req.branch_pickup_id,
                    rounding_delta_egp: checkout_summary.rounding_delta_egp,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                });
            } else if status.as_deref() == Some("409") {
                return Err(AppError::new(
                    ErrorCode::Conflict,
                    "Payload mismatch for idempotency key",
                ));
            }
        }

        // Create order in transaction
        let mut tx = db.begin().await.map_err(|e| AppError::database_error(&e))?;

        let order_id = Uuid::new_v4();
        let public_order_number = Self::generate_public_order_number(&mut tx, store_id)
            .await?;

        let now = Utc::now();

        // Build order snapshot per D-114
        let order_snapshot = OrderSnapshot {
            lines: checkout_summary.line_items.iter().map(|item| {
                OrderLineSnapshot {
                    product_id: item.product_id.clone(),
                    product_name: item.product_name.clone(),
                    product_image_s3_key: None, // TODO: fetch from product
                    variant_id: item.variant_id.clone(),
                    variant_name: item.variant_name.clone(),
                    sku: None, // TODO: fetch
                    unit_price_egp: item.unit_price_egp,
                    quantity: item.quantity,
                    line_total_egp: item.line_total_egp,
                    line_tax_egp: item.line_tax_egp,
                    branch_id: "".to_string(), // TODO: from line
                }
            }).collect(),
            subtotal_egp: checkout_summary.subtotal_egp,
            tax_egp: checkout_summary.tax_egp,
            shipping_cost_egp: checkout_summary.shipping_cost_egp,
            total_egp: checkout_summary.total_egp,
            tax_rate_bp: 0, // TODO: fetch from settings
        };

        let snapshot_json = serde_json::to_value(&order_snapshot)
            .map_err(|e| AppError::new(ErrorCode::InternalError, &e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO app.order (id, store_id, public_order_number, consumer_email, consumer_phone, verification_method, order_snapshot, order_status, payment_status, delivery_governorate, delivery_county, delivery_district, delivery_address, delivery_coords_lat, delivery_coords_lng, branch_pickup_id, rounding_delta_egp, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, 'email_only', $6, 'pending', 'delayed', $7, $8, $9, $10, $11, $12, $13, $14, $15, $15)
            "#,
        )
        .bind(&order_id)
        .bind(&store_id)
        .bind(&public_order_number)
        .bind(consumer_email)
        .bind(&req.phone)
        .bind(&snapshot_json)
        .bind(&req.delivery_governorate)
        .bind(&req.delivery_county)
        .bind(&req.delivery_district)
        .bind(&req.delivery_address)
        .bind(req.delivery_coords_lat)
        .bind(req.delivery_coords_lng)
        .bind(&req.branch_pickup_id)
        .bind(checkout_summary.rounding_delta_egp)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Create pending stock reserves per D-051
        let reserve_lines: Vec<_> = cart.lines.iter().map(|l| {
            (l.product_id, l.variant_id, l.branch_id, l.quantity)
        }).collect();

        StockReserveManager::create_pending_reserves(&mut *tx, order_id, reserve_lines)
            .await?;

        // Record idempotency key
        let response_json = json!({
            "id": order_id.to_string(),
            "public_order_number": public_order_number,
            "status": "pending",
        });

        sqlx::query(
            r#"
            INSERT INTO app.idempotency_key (principal_type, principal_id, key, payload_hash, response_status, response_body, created_at)
            VALUES ($1, $2, $3, $4, '201', $5, $6)
            ON CONFLICT (principal_type, principal_id, key) DO NOTHING
            "#,
        )
        .bind(principal_type)
        .bind(principal_id)
        .bind(idempotency_key)
        .bind(&payload_hash_hex)
        .bind(response_json.to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        Ok(Order {
            id: order_id,
            store_id,
            public_order_number,
            consumer_email: consumer_email.to_string(),
            consumer_phone: req.phone,
            verification_method: "email_only".to_string(),
            order_snapshot: snapshot_json,
            order_status: "pending".to_string(),
            payment_status: "delayed".to_string(),
            delivery_governorate: req.delivery_governorate,
            delivery_county: req.delivery_county,
            delivery_district: req.delivery_district,
            delivery_address: req.delivery_address,
            delivery_coords_lat: req.delivery_coords_lat,
            delivery_coords_lng: req.delivery_coords_lng,
            branch_pickup_id: req.branch_pickup_id,
            rounding_delta_egp: checkout_summary.rounding_delta_egp,
            created_at: now,
            updated_at: now,
        })
    }

    /// Generate public order number per D-120: <slug>-<base32-6>
    async fn generate_public_order_number(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        store_id: Uuid,
    ) -> Result<String, AppError> {
        let slug: (String,) = sqlx::query_as(
            "SELECT slug FROM app.store WHERE id = $1",
        )
        .bind(&store_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let base32_suffix = Self::random_base32_6chars();
        Ok(format!("{}-{}", slug.0, base32_suffix))
    }

    fn random_base32_6chars() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut rng = rand::thread_rng();

        (0..6)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Get order by ID
    pub async fn get_order(
        db: &PgPool,
        store_id: Uuid,
        order_id: Uuid,
    ) -> Result<Order, AppError> {
        sqlx::query_as::<_, Order>(
            "SELECT * FROM app.order WHERE id = $1 AND store_id = $2",
        )
        .bind(&order_id)
        .bind(&store_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Order not found"))
    }

    /// List orders for store
    pub async fn list_orders(
        db: &PgPool,
        store_id: Uuid,
    ) -> Result<Vec<Order>, AppError> {
        sqlx::query_as::<_, Order>(
            "SELECT * FROM app.order WHERE store_id = $1 ORDER BY created_at DESC LIMIT 100",
        )
        .bind(&store_id)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Confirm order: transition reserves to active, decrement stock per D-051
    pub async fn confirm_order(
        db: &PgPool,
        store_id: Uuid,
        order_id: Uuid,
    ) -> Result<(), AppError> {
        // Activate stock reserves and decrement
        StockReserveManager::activate_reserves_and_decrement_stock(db, order_id).await?;

        // Update order status to confirmed-category (e.g., "confirmed")
        sqlx::query(
            "UPDATE app.order SET order_status = 'confirmed', updated_at = now() WHERE id = $1 AND store_id = $2",
        )
        .bind(&order_id)
        .bind(&store_id)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(())
    }

    /// Cancel order: release stock per D-051
    pub async fn cancel_order(
        db: &PgPool,
        store_id: Uuid,
        order_id: Uuid,
    ) -> Result<(), AppError> {
        StockReserveManager::release_stock(db, order_id).await?;

        sqlx::query(
            "UPDATE app.order SET order_status = 'cancelled', updated_at = now() WHERE id = $1 AND store_id = $2",
        )
        .bind(&order_id)
        .bind(&store_id)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(())
    }
}
