use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PuckPage {
    pub id: Uuid,
    pub store_id: Uuid,
    pub slug: String,
    pub title: String,
    pub state: String, // draft, published, archived
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PuckPageRevision {
    pub id: Uuid,
    pub page_id: Uuid,
    pub content: serde_json::Value,
    pub revision_number: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PuckPagePublish {
    pub id: Uuid,
    pub page_id: Uuid,
    pub revision_id: Uuid,
    pub is_rollback: bool,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PagePreviewToken {
    pub id: Uuid,
    pub page_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
