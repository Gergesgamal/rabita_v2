use crate::error::{AppError, ErrorCode};
use crate::cart::{Cart, CartManager};
use crate::shipping::ShippingManager;
use sqlx::PgPool;
use uuid::Uuid;

use super::models::*;

pub struct CheckoutManager;

impl CheckoutManager {
    /// Validate cart before checkout per D-114, D-115, D-119
    pub async fn validate_cart(
        db: &PgPool,
        store_id: Uuid,
        cart: &Cart,
        delivery_governorate: &str,
        delivery_county: &str,
        delivery_district: &str,
        branch_id_for_pickup: Option<Uuid>,
    ) -> Result<CheckoutCartValidation, AppError> {
        let mut errors = vec![];
        let mut warnings = vec![];

        if cart.lines.is_empty() {
            return Err(AppError::new(
                ErrorCode::InvalidState,
                "Cart is empty",
            ));
        }

        // Check each line for availability and pricing
        for line in &cart.lines {
            // Verify product exists and is published
            let product: Option<(bool, i64)> = sqlx::query_as(
                "SELECT is_published, base_price_egp FROM app.product WHERE id = $1 AND store_id = $2 AND deleted_at IS NULL",
            )
            .bind(&line.product_id)
            .bind(&store_id)
            .fetch_optional(db)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            let (is_published, base_price) = match product {
                Some((published, price)) if published => (published, price),
                _ => {
                    errors.push(CheckoutValidationError {
                        product_id: line.product_id.to_string(),
                        reason: "Product not available".to_string(),
                    });
                    continue;
                }
            };

            // Check branch product stock
            let stock: Option<(i64,)> = sqlx::query_as(
                "SELECT stock_qty FROM app.branch_product WHERE branch_id = $1 AND product_id = $2",
            )
            .bind(&line.branch_id)
            .bind(&line.product_id)
            .fetch_optional(db)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            if let Some((qty,)) = stock {
                if qty < line.quantity {
                    errors.push(CheckoutValidationError {
                        product_id: line.product_id.to_string(),
                        reason: format!("Only {} in stock", qty),
                    });
                }
            } else {
                errors.push(CheckoutValidationError {
                    product_id: line.product_id.to_string(),
                    reason: "Not available at selected branch".to_string(),
                });
            }
        }

        // Check shipping availability per D-119
        if branch_id_for_pickup.is_none() {
            let shipping_rule = ShippingManager::find_matching_rule(
                db,
                store_id,
                None,
                delivery_governorate,
                delivery_county,
                delivery_district,
            )
            .await?;

            if shipping_rule.is_none() {
                errors.push(CheckoutValidationError {
                    product_id: "shipping".to_string(),
                    reason: "Shipping not available to this address".to_string(),
                });
            }
        }

        Ok(CheckoutCartValidation {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        })
    }

    /// Calculate checkout summary with tax, shipping, promotions per D-053, D-114, D-118
    pub async fn calculate_checkout(
        db: &PgPool,
        store_id: Uuid,
        cart: &Cart,
        delivery_governorate: &str,
        delivery_county: &str,
        delivery_district: &str,
        branch_pickup_id: Option<Uuid>,
        coupon_code: Option<&str>,
    ) -> Result<CheckoutSummary, AppError> {
        let mut subtotal_egp: i64 = 0;
        let mut line_items = vec![];

        // Fetch current prices and compute subtotal
        for line in &cart.lines {
            let (product_name, base_price): (String, i64) = sqlx::query_as(
                "SELECT name, base_price_egp FROM app.product WHERE id = $1 AND store_id = $2",
            )
            .bind(&line.product_id)
            .bind(&store_id)
            .fetch_one(db)
            .await
            .map_err(|e| AppError::database_error(&e))?;

            let unit_price = if let Some(variant_id) = line.variant_id {
                let override_price: Option<(Option<i64>,)> = sqlx::query_as(
                    "SELECT price_override_egp FROM app.variant WHERE id = $1 AND product_id = $2",
                )
                .bind(&variant_id)
                .bind(&line.product_id)
                .fetch_optional(db)
                .await
                .map_err(|e| AppError::database_error(&e))?;

                override_price
                    .and_then(|(p,)| p)
                    .unwrap_or(base_price)
            } else {
                base_price
            };

            let line_total = unit_price * line.quantity;
            subtotal_egp += line_total;

            let variant_name: Option<String> = if let Some(variant_id) = line.variant_id {
                sqlx::query_scalar("SELECT name FROM app.variant WHERE id = $1")
                    .bind(&variant_id)
                    .fetch_optional(db)
                    .await
                    .map_err(|e| AppError::database_error(&e))?
            } else {
                None
            };

            line_items.push(CheckoutLineItem {
                product_id: line.product_id.to_string(),
                product_name,
                variant_id: line.variant_id.map(|v| v.to_string()),
                variant_name,
                quantity: line.quantity,
                unit_price_egp: unit_price,
                line_total_egp: line_total,
                line_tax_egp: 0, // Will be calculated per line
            });
        }

        // Get tax rate per D-118 (tax-exclusive per D-053)
        let tax_rate_bp: i32 = sqlx::query_scalar(
            "SELECT tax_rate_bp FROM app.store_settings WHERE store_id = $1",
        )
        .bind(&store_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Calculate tax per line
        let mut total_tax_egp: i64 = 0;
        for item in &mut line_items {
            let line_tax = (item.line_total_egp * tax_rate_bp as i64) / 10000;
            item.line_tax_egp = line_tax;
            total_tax_egp += line_tax;
        }

        // Get shipping cost per D-119
        let shipping_cost_egp = if let Some(branch_id) = branch_pickup_id {
            0 // Pickup is free
        } else {
            let rule = ShippingManager::find_matching_rule(
                db,
                store_id,
                None,
                delivery_governorate,
                delivery_county,
                delivery_district,
            )
            .await?
            .ok_or_else(|| AppError::new(
                ErrorCode::InvalidInput,
                "Shipping not available to this address",
            ))?;

            rule.cost_egp
        };

        // Apply coupon if provided (TODO: promotion application logic)
        let mut discount_egp: i64 = 0;
        if let Some(code) = coupon_code {
            // TODO: Look up coupon, verify validity, apply discount
        }

        // Calculate total with rounding per D-053 (round to nearest EGP at final total)
        let subtotal_with_tax = subtotal_egp + total_tax_egp;
        let total_before_rounding = subtotal_with_tax + shipping_cost_egp - discount_egp;
        let total_egp = ((total_before_rounding + 50) / 100) * 100; // Round to nearest EGP
        let rounding_delta_egp = total_egp - total_before_rounding;

        Ok(CheckoutSummary {
            line_items,
            subtotal_egp,
            discount_egp,
            tax_egp: total_tax_egp,
            shipping_cost_egp,
            total_egp,
            rounding_delta_egp,
        })
    }
}
