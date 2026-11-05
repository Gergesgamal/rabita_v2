use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RefundRequest {
    pub id: Uuid,
    pub order_id: Uuid,
    pub consumer_id: Option<Uuid>, // NULL for guest (claimed or not claimed)
    pub reason: String,
    pub refund_amount_egp: i64, // Full refund only per D-121
    pub state: String,          // pending, approved, rejected, escalated_admin, issued
    pub merchant_decision: Option<String>, // approved, rejected
    pub merchant_decision_reason: Option<String>,
    pub admin_decision: Option<String>,
    pub escalated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRefundRequestReq {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct ApproveRefundRequest {
    pub approved: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RefundResponse {
    pub id: String,
    pub state: String,
    pub refund_amount_egp: i64,
    pub reason: String,
    pub created_at: String,
}
