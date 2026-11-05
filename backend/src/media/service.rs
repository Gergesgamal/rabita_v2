// backend/src/media/service.rs

//! Media service per D-037, D-104: quota enforcement, GuardDuty scanning.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct MediaManager;

impl MediaManager {
    /// Request presigned upload URL per D-037.
    /// Validates against quota and per-file limits before issuing URL.
    pub async fn request_upload_url(
        db: &PgPool,
        store_id: Uuid,
        media_type: &str,
        declared_size_bytes: i64,
    ) -> AppResult<(String, Uuid)> {
        // Validate media type
        if !matches!(media_type, "image" | "video" | "pdf" | "other") {
            return Err(AppError::BadRequest("Invalid media type".to_string()));
        }

        // Check file size limits per D-037
        let max_size = match media_type {
            "image" => 20 * 1024 * 1024, // 20 MB
            "video" => 100 * 1024 * 1024, // 100 MB
            "pdf" => 20 * 1024 * 1024, // 20 MB
            _ => 50 * 1024 * 1024, // 50 MB
        };

        if declared_size_bytes > max_size {
            return Err(AppError::BadRequest(format!(
                "File size exceeds limit for {}",
                media_type
            )));
        }

        // Check quota per D-037
        let quota: (i64, i64) = sqlx::query_as(
            r#"
            SELECT quota_bytes, used_bytes FROM app.media_quota WHERE store_id = $1
            "#,
        )
        .bind(store_id)
        .fetch_one(db)
        .await?;

        let (quota_bytes, used_bytes) = quota;

        if used_bytes + declared_size_bytes > quota_bytes {
            return Err(AppError::BadRequest("Quota exceeded".to_string()));
        }

        // Create media record in 'pending' state
        let media_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.media (store_id, media_type, file_size_bytes, guarduty_scan_status)
            VALUES ($1, $2, $3, 'pending')
            RETURNING id
            "#,
        )
        .bind(store
