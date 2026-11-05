use crate::error::{AppError, ErrorCode};
use crate::state::AppState;
use redis::AsyncCommands;
use uuid::Uuid;
use chrono::Utc;

use super::models::*;

pub struct CartManager;

impl CartManager {
    const CART_TTL_SECONDS: u32 = 30 * 24 * 60 * 60; // 30 days per D-115

    /// Get cart key for consumer or guest session
    fn get_cart_key(store_id: Uuid, consumer_id_or_session: &str) -> String {
        format!("cart:{}:{}", store_id, consumer_id_or_session)
    }

    /// Get consumer's cart from Valkey per D-115
    pub async fn get_cart(
        cache: &mut redis::aio::ConnectionManager,
        store_id: Uuid,
        consumer_id: &str,
    ) -> Result<Cart, AppError> {
        let key = Self::get_cart_key(store_id, consumer_id);
        
        let cart_json: Option<String> = cache
            .get(&key)
            .await
            .map_err(|e| AppError::new(
                ErrorCode::CacheError,
                &format!("Valkey error: {}", e),
            ))?;

        if let Some(json) = cart_json {
            let cart: Cart = serde_json::from_str(&json).map_err(|e| {
                AppError::new(ErrorCode::InternalError, &format!("JSON parse error: {}", e))
            })?;
            Ok(cart)
        } else {
            // Empty cart
            Ok(Cart {
                store_id,
                lines: vec![],
                updated_at: Utc::now(),
            })
        }
    }

    /// Add item to cart per D-115
    pub async fn add_to_cart(
        cache: &mut redis::aio::ConnectionManager,
        store_id: Uuid,
        consumer_id: &str,
        req: AddToCartRequest,
    ) -> Result<Cart, AppError> {
        if req.quantity <= 0 {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "Quantity must be positive",
            ));
        }

        let key = Self::get_cart_key(store_id, consumer_id);
        let mut cart = Self::get_cart(cache, store_id, consumer_id).await?;

        // Find or create line
        let line = cart.lines.iter_mut().find(|l| {
            l.product_id == req.product_id
                && l.variant_id == req.variant_id
                && l.branch_id == req.branch_id
        });

        if let Some(line) = line {
            line.quantity += req.quantity;
        } else {
            cart.lines.push(CartLine {
                product_id: req.product_id,
                variant_id: req.variant_id,
                branch_id: req.branch_id,
                quantity: req.quantity,
                unit_price_egp: 0, // Will be set by checkout validation
                added_at: Utc::now(),
            });
        }

        cart.updated_at = Utc::now();

        // Persist to Valkey with TTL
        let json = serde_json::to_string(&cart).map_err(|e| {
            AppError::new(ErrorCode::InternalError, &format!("JSON error: {}", e))
        })?;

        cache
            .set_ex::<_, _, ()>(&key, json, Self::CART_TTL_SECONDS as u64)
            .await
            .map_err(|e| AppError::new(
                ErrorCode::CacheError,
                &format!("Valkey error: {}", e),
            ))?;

        Ok(cart)
    }

    /// Remove item from cart
    pub async fn remove_from_cart(
        cache: &mut redis::aio::ConnectionManager,
        store_id: Uuid,
        consumer_id: &str,
        req: RemoveFromCartRequest,
    ) -> Result<Cart, AppError> {
        let key = Self::get_cart_key(store_id, consumer_id);
        let mut cart = Self::get_cart(cache, store_id, consumer_id).await?;

        cart.lines.retain(|l| {
            !(l.product_id == req.product_id
                && l.variant_id == req.variant_id
                && l.branch_id == req.branch_id)
        });

        cart.updated_at = Utc::now();

        let json = serde_json::to_string(&cart).map_err(|e| {
            AppError::new(ErrorCode::InternalError, &format!("JSON error: {}", e))
        })?;

        cache
            .set_ex::<_, _, ()>(&key, json, Self::CART_TTL_SECONDS as u64)
            .await
            .map_err(|e| AppError::new(
                ErrorCode::CacheError,
                &format!("Valkey error: {}", e),
            ))?;

        Ok(cart)
    }

    /// Merge guest cart into account cart per D-115
    pub async fn merge_guest_cart(
        cache: &mut redis::aio::ConnectionManager,
        store_id: Uuid,
        guest_session_id: &str,
        consumer_id: &str,
    ) -> Result<Cart, AppError> {
        let guest_cart = Self::get_cart(cache, store_id, guest_session_id).await?;
        let mut account_cart = Self::get_cart(cache, store_id, consumer_id).await?;

        // Merge: account values win on conflicts, non-conflicting guest lines append
        for guest_line in guest_cart.lines {
            let existing = account_cart.lines.iter_mut().find(|l| {
                l.product_id == guest_line.product_id
                    && l.variant_id == guest_line.variant_id
                    && l.branch_id == guest_line.branch_id
            });

            if existing.is_none() {
                account_cart.lines.push(guest_line);
            }
        }

        account_cart.updated_at = Utc::now();

        // Save merged cart to account key
        let key = Self::get_cart_key(store_id, consumer_id);
        let json = serde_json::to_string(&account_cart).map_err(|e| {
            AppError::new(ErrorCode::InternalError, &format!("JSON error: {}", e))
        })?;

        cache
            .set_ex::<_, _, ()>(&key, json, Self::CART_TTL_SECONDS as u64)
            .await
            .map_err(|e| AppError::new(
                ErrorCode::CacheError,
                &format!("Valkey error: {}", e),
            ))?;

        // Clear guest cart
        let guest_key = Self::get_cart_key(store_id, guest_session_id);
        let _: redis::RedisResult<()> = cache.del::<_, ()>(&guest_key).await;

        Ok(account_cart)
    }

    /// Clear cart
    pub async fn clear_cart(
        cache: &mut redis::aio::ConnectionManager,
        store_id: Uuid,
        consumer_id: &str,
    ) -> Result<(), AppError> {
        let key = Self::get_cart_key(store_id, consumer_id);
        cache.del::<_, ()>(&key).await.map_err(|e| {
            AppError::new(
                ErrorCode::CacheError,
                &format!("Valkey error: {}", e),
            )
        })?;
        Ok(())
    }
}
