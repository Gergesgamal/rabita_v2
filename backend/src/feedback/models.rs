use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FeedbackItem {
    pub id: Uuid,
    pub account_id: Uuid,
    pub store_id: Option<Uuid>, // NULL for bugs (never public per D-077)
    pub category: String,        // feature, improvement, bug, util
    pub title: String,
    pub description: String,
    pub state: String, // open, under_review, planned, in_progress, shipped, declined
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFeedbackRequest {
    pub category: String, // feature, improvement, bug, util
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct FeedbackResponse {
    pub id: String,
    pub category: String,
    pub title: String,
    pub state: String,
    pub created_at: String,
}
