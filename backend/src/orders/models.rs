use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::types::JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub store_id: Uuid,
    pub public_order_number: String, // <slug>-<base32-6> per D-120
    pub consumer_email: String,
    pub consumer_phone: Option<String>,
    pub verification_method: String, // email_only, whatsapp_otp, phone_not_on_whatsapp
    pub order_snapshot: JsonValue, // Immutable per D-114
    pub order_status: String,       // Platform category
    pub payment_status: String,
    pub delivery_governorate: String,
    pub delivery_county: String,
    pub delivery_district: String,
    pub delivery_address: String,
    pub delivery_coords_lat: Option<f64>,
    pub delivery_coords_lng: Option<f64>,
    pub branch_pickup_id: Option<Uuid>,
    pub rounding_delta_egp: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSnapshot {
    pub lines: Vec<OrderLineSnapshot>,
    pub subtotal_egp: i64,
    pub tax_egp: i64,
    pub shipping_cost_egp: i64,
    pub total_egp: i64,
    pub tax_rate_bp: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineSnapshot {
    pub product_id: String,
    pub product_name: String,
    pub product_image_s3_key: Option<String>,
    pub variant_id: Option<String>,
    pub variant_name: Option<String>,
    pub sku: Option<String>,
    pub unit_price_egp: i64,
    pub quantity: i64,
    pub line_total_egp: i64,
    pub line_tax_egp: i64,
    pub branch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub quantity: i64,
    pub unit_price_egp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockReserve {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub branch_id: Uuid,
    pub quantity: i64,
    pub state: String, // pending, active, released, shadowed
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub email: String,
    pub phone: Option<String>,
    pub delivery_governorate: String,
    pub delivery_county: String,
    pub delivery_district: String,
    pub delivery_address: String,
    pub delivery_coords_lat: Option<f64>,
    pub delivery_coords_lng: Option<f64>,
    pub branch_pickup_id: Option<Uuid>,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub public_order_number: String,
    pub status: String,
    pub payment_status: String,
    pub total_egp: i64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
    pub note: Option<String>,
}
