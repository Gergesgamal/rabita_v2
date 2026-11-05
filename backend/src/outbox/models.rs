use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OutboxEvent {
    pub id: Uuid,
    pub event_type: String, // order_created, order_confirmed, refund_approved, ticket_created, etc.
    pub aggregate_id: Uuid,  // order_id, etc.
    pub payload: serde_json::Value,
    pub state: String, // pending, processing, completed, failed, dlq
    pub lease_until: Option<DateTime<Utc>>, // For crash recovery per D-084
    pub attempt_count: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OutboxAttempt {
    pub id: Uuid,
    pub event_id: Uuid,
    pub attempt_number: i32,
    pub status: String, // success, failure
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}
