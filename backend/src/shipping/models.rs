use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShippingRule {
    pub id: Uuid,
    pub store_id: Uuid,
    pub branch_id: Option<Uuid>, // NULL for store-wide
    pub governorate: Option<String>,
    pub county: Option<String>,
    pub district: Option<String>,
    pub cost_egp: i64, // piastres
    pub estimated_days: i32,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShippingPolicy {
    pub id: Uuid,
    pub store_id: Uuid,
    pub title: String,
    pub content: String, // sanitized HTML
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeliverySlotConfig {
    pub id: Uuid,
    pub store_id: Uuid,
    pub allow_consumer_date_selection: bool,
    pub delivery_window_days_min: i32,
    pub delivery_window_days_max: i32,
}

#[derive(Debug, Deserialize)]
#[derive(serde::Serialize)]
pub struct CreateShippingRuleRequest {
    pub branch_id: Option<Uuid>,
    pub governorate: Option<String>,
    pub county: Option<String>,
    pub district: Option<String>,
    pub cost_egp: i64,
    pub estimated_days: i32,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateShippingPolicyRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeliverySlotConfigRequest {
    pub allow_consumer_date_selection: Option<bool>,
    pub delivery_window_days_min: Option<i32>,
    pub delivery_window_days_max: Option<i32>,
}
