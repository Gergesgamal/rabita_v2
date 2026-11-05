use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobPost {
    pub id: Uuid,
    pub store_id: Uuid,
    pub title: String,
    pub description: String,
    pub category: String,
    pub contact_phone: String,
    pub state: String, // draft, public, unpublished, archived
    pub is_confidential: bool, // noindex if true per D-098
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobApplication {
    pub id: Uuid,
    pub job_id: Uuid,
    pub applicant_name: String,
    pub applicant_email: String,
    pub state: String, // applied, reviewed, accepted, rejected, withdrawn
    pub cv_media_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobChatThread {
    pub id: Uuid,
    pub job_id: Uuid,
    pub application_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobChatMessage {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub is_from_merchant: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJobPostRequest {
    pub title: String,
    pub description: String,
    pub category: String,
    pub contact_phone: String,
    pub is_confidential: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitApplicationRequest {
    pub applicant_name: String,
    pub applicant_email: String,
    pub cv_media_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct SendChatMessageRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct JobPostResponse {
    pub id: String,
    pub title: String,
    pub state: String,
    pub created_at: String,
}
