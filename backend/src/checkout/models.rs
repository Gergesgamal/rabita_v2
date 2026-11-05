use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub email: String,
    pub phone: Option<String>,
    pub delivery_governorate: String,
    pub delivery_county: String,
    pub delivery_district: String,
    pub delivery_address: String,
    pub delivery_coords_lat: Option<f64>,
    pub delivery_coords_lng: Option<f64>,
    pub branch_pickup_id: Option<Uuid>, // If pickup
    pub coupon_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckoutCartValidation {
    pub is_valid: bool,
    pub errors: Vec<CheckoutValidationError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckoutValidationError {
    pub product_id: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct CheckoutSummary {
    pub line_items: Vec<CheckoutLineItem>,
    pub subtotal_egp: i64, // piastres
    pub discount_egp: i64,
    pub tax_egp: i64,
    pub shipping_cost_egp: i64,
    pub total_egp: i64,
    pub rounding_delta_egp: i64, // Per D-053
}

#[derive(Debug, Serialize)]
pub struct CheckoutLineItem {
    pub product_id: String,
    pub product_name: String,
    pub variant_id: Option<String>,
    pub variant_name: Option<String>,
    pub quantity: i64,
    pub unit_price_egp: i64,
    pub line_total_egp: i64,
    pub line_tax_egp: i64,
}

#[derive(Debug, Serialize)]
pub struct VerificationMethod {
    pub method: String, // email_only, whatsapp_otp, phone_not_on_whatsapp
    pub phone_number: Option<String>,
    pub email: String,
}
