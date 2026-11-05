use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartLine {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub branch_id: Uuid,
    pub quantity: i64,
    pub unit_price_egp: i64, // Current price in piastres
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub store_id: Uuid,
    pub lines: Vec<CartLine>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub branch_id: Uuid,
    pub quantity: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartLineRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub branch_id: Uuid,
    pub quantity: i64,
}

#[derive(Debug, Deserialize)]
pub struct RemoveFromCartRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub branch_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub lines: Vec<CartLineResponse>,
    pub line_count: i32,
}

#[derive(Debug, Serialize)]
pub struct CartLineResponse {
    pub product_id: String,
    pub variant_id: Option<String>,
    pub branch_id: String,
    pub quantity: i64,
    pub unit_price_egp: i64,
}

#[derive(Debug, Clone)]
pub struct CartValidationError {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub error: String,
}
