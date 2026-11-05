use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub order_id: Option<Uuid>, // Mandatory when opened from order per D-094
    pub store_id: Option<Uuid>, // NULL for platform tickets (admin-scoped)
    pub consumer_id: Uuid,
    pub subject: String,
    pub state: String, // pending_support, pending_user, resolved, closed, reopened
    pub reopen_deadline: Option<DateTime<Utc>>, // 7 days after close per D-094
    pub lifetime_seconds: i32, // 14-day default per D-095
    pub inactivity_seconds: i32, // Current inactivity window
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TicketReply {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub author_id: Uuid, // Consumer or employee/admin ID
    pub content: String,
    pub is_consumer: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TicketAttachment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub media_id: Uuid,
    pub file_name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>, // 3 days after ticket close per D-094
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TicketLastViewed {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub actor_id: Uuid,
    pub last_viewed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[derive(serde::Serialize)]
pub struct CreateTicketRequest {
    pub subject: String,
    pub content: String,
    pub order_id: Option<Uuid>, // Must be provided if opened from order context
}

#[derive(Debug, Deserialize)]
pub struct ReplyToTicketRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct TicketResponse {
    pub id: String,
    pub subject: String,
    pub state: String,
    pub created_at: String,
}
