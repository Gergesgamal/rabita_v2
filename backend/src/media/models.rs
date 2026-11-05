use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Media {
    pub id: Uuid,
    pub store_id: Uuid,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub s3_key_staging: Option<String>,
    pub s3_key_media: Option<String>,
    pub guardduty_scan_status: String, // pending, no_threats_found, threats_found, unsupported, access_denied, failed
    pub is_promoted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaQuota {
    pub store_id: Uuid,
    pub quota_bytes: i64,
    pub used_bytes: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GetUploadUrlRequest {
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
}

#[derive(Debug, Serialize)]
pub struct UploadUrlResponse {
    pub upload_url: String,
    pub media_id: String,
    pub s3_key: String,
}

#[derive(Debug, Serialize)]
pub struct MediaResponse {
    pub id: String,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub is_promoted: bool,
    pub scan_status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct QuotaResponse {
    pub quota_bytes: i64,
    pub used_bytes: i64,
    pub percent_used: f64,
}
