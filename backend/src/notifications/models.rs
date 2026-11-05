use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationLog {
    pub id: Uuid,
    pub consumer_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub notification_type: String, // order_placed, merchant_confirmed, shipped, delivered, refund_approved, support_reply
    pub email_address: String,
    pub delivery_status: String, // queued, sent, delivered, bounced, complained, failed
    pub ses_message_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotification {
    pub notification_type: String,
    pub recipient_email: String,
    pub subject: String,
    pub html_body: String,
}
