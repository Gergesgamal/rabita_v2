use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Discount {
    pub id: Uuid,
    pub store_id: Uuid,
    pub product_id: Uuid,
    pub discount_amount_egp: Option<i64>,     // EGP
    pub discount_percent_bp: Option<i32>,     // Basis points
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Coupon {
    pub id: Uuid,
    pub store_id: Uuid,
    pub code: String,
    pub discount_amount_egp: Option<i64>,
    pub discount_percent_bp: Option<i32>,
    pub branch_id: Option<Uuid>, // NULL for store-wide
    pub is_active: bool,
    pub max_uses: Option<i32>,
    pub current_uses: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Bundle {
    pub id: Uuid,
    pub store_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub discount_amount_egp: Option<i64>,
    pub discount_percent_bp: Option<i32>,
    pub allow_partial_removal: bool,
    pub max_partial_applications: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BundleItem {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub product_id: Uuid,
    pub required_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BundlePartialApplication {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub order_id: Uuid,
    pub removed_product_ids: Vec<String>, // JSON array of product IDs
    pub original_price_egp: i64,
    pub recomputed_price_egp: i64,
    pub discount_applied_egp: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub discount_amount_egp: Option<i64>,
    pub discount_percent_bp: Option<i32>,
    pub branch_id: Option<Uuid>,
    pub max_uses: Option<i32>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBundleRequest {
    pub name: String,
    pub description: Option<String>,
    pub discount_amount_egp: Option<i64>,
    pub discount_percent_bp: Option<i32>,
    pub allow_partial_removal: Option<bool>,
    pub max_partial_applications: Option<i32>,
    pub product_ids: Vec<Uuid>,
}
