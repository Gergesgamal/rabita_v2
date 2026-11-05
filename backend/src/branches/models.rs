use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Branch {
    pub id: Uuid,
    pub store_id: Uuid,
    pub name: String,
    pub address: String,
    pub governorate: String,
    pub county: String,
    pub district: String,
    pub phone: String,
    pub is_published: bool,
    pub allow_pickup: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BranchWorkingHours {
    pub id: Uuid,
    pub branch_id: Uuid,
    pub day_of_week: i32, // 0 = Monday, 6 = Sunday
    pub open_time: Option<String>, // HH:MM or NULL if closed
    pub close_time: Option<String>,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BranchProduct {
    pub id: Uuid,
    pub branch_id: Uuid,
    pub product_id: Uuid,
    pub stock_qty: i64, // CHECK (stock_qty >= 0)
    pub is_published: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBranchRequest {
    pub name: String,
    pub address: String,
    pub governorate: String,
    pub county: String,
    pub district: String,
    pub phone: String,
    pub allow_pickup: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub governorate: Option<String>,
    pub county: Option<String>,
    pub district: Option<String>,
    pub phone: Option<String>,
    pub allow_pickup: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SetWorkingHoursRequest {
    pub hours: Vec<WorkingHourInput>,
}

#[derive(Debug, Deserialize)]
pub struct WorkingHourInput {
    pub day_of_week: i32,
    pub open_time: Option<String>, // "09:00"
    pub close_time: Option<String>,
    pub is_closed: bool,
}

#[derive(Debug, Deserialize)]
pub struct AdjustStockRequest {
    pub product_id: Uuid,
    pub quantity_delta: i64,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct PublishBranchProductRequest {
    pub product_ids: Vec<Uuid>,
    pub publish: bool,
}

#[derive(Debug, Serialize)]
pub struct BranchResponse {
    pub id: String,
    pub store_id: String,
    pub name: String,
    pub address: String,
    pub governorate: String,
    pub county: String,
    pub district: String,
    pub phone: String,
    pub is_published: bool,
    pub allow_pickup: bool,
    pub working_hours: Vec<BranchWorkingHours>,
    pub created_at: String,
}
