use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PuckComponent {
    pub id: Uuid,
    pub store_id: Option<Uuid>, // NULL for platform-wide
    pub name: String,
    pub approval_state: String, // pending_approval, active, rejected
    pub source_code: String,    // TSX/JSX
    pub compiled_code: Option<String>,
    pub rejection_reason: Option<String>,
    pub author_account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ComponentApproval {
    pub id: Uuid,
    pub component_id: Uuid,
    pub approver_account_id: Uuid,
    pub decision: String, // approved, rejected
    pub feedback: Option<String>,
    pub created_at: DateTime<Utc>,
}
