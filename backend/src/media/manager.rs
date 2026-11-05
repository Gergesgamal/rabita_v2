use crate::error::{AppError, ErrorCode};
use crate::audit::AuditLogger;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use std::sync::Arc;

use super::models::*;

pub struct MediaManager;

impl MediaManager {
    const DEFAULT_QUOTA_BYTES: i64 = 3_000_000_000; // 3 GB
    const MAX_FILE_SIZE: i64 = 100_000_000; // 100 MB for video
    const NEW_ACCOUNT_DAILY_LIMIT: i64 = 500_000_000; // 500 MB

    /// Create a presigned upload URL for a media object.
    pub async fn create_upload_url(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        req: GetUploadUrlRequest,
    ) -> Result<UploadUrlResponse, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        // Check quota
        let quota: MediaQuota = sqlx::query_as(
            "SELECT store_id, quota_bytes, used_bytes, updated_at FROM app.media_quota WHERE store_id = $1"
        )
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .unwrap_or(MediaQuota {
            store_id,
            quota_bytes: Self::DEFAULT_QUOTA_BYTES,
            used_bytes: 0,
            updated_at: Utc::now(),
        });

        if quota.used_bytes + req.file_size > quota.quota_bytes {
            return Err(AppError::new(
                ErrorCode::QuotaExceeded,
                "Upload would exceed quota",
            ));
        }

        if req.file_size > Self::MAX_FILE_SIZE {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "File size exceeds maximum",
            ));
        }

        // Create media record
        let media_id = Uuid::new_v4();
        let s3_key = format!("staging/{}/{}/{}", store_id, Uuid::new_v4(), req.file_name);
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.media (id, store_id, file_name, file_size, mime_type, s3_key_staging, guardduty_scan_status, is_promoted, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, 'pending', false, $7, $7)
            "#,
        )
        .bind(&media_id)
        .bind(&store_id)
        .bind(&req.file_name)
        .bind(req.file_size)
        .bind(&req.mime_type)
        .bind(&s3_key)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // TODO: Generate presigned URL from S3Manager
        // For now, return stub
        let upload_url = format!("https://s3.amazonaws.com/rabita-staging/{}?signed", s3_key);

        Ok(UploadUrlResponse {
            upload_url,
            media_id: media_id.to_string(),
            s3_key,
        })
    }

    /// Handle GuardDuty scan status change.
    /// Called by EventBridge on scan_status tag change.
    pub async fn on_scan_status_changed(
        pool: &PgPool,
        media_id: Uuid,
        s3_key: String,
        scan_status: &str,
    ) -> Result<(), AppError> {
        match scan_status {
            "NO_THREATS_FOUND" => {
                // Promote to media bucket
                Self::promote_to_media(pool, media_id, &s3_key).await?;
            }
            "THREATS_FOUND" => {
                // Move to quarantine
                Self::quarantine_media(pool, media_id, &s3_key).await?;
            }
            "UNSUPPORTED" | "ACCESS_DENIED" | "FAILED" => {
                // Hold and alert
                Self::hold_media(pool, media_id, scan_status).await?;
            }
            _ => {
                // Unknown status: default-deny
                Self::hold_media(pool, media_id, "UNKNOWN").await?;
            }
        }
        Ok(())
    }

    async fn promote_to_media(
        pool: &PgPool,
        media_id: Uuid,
        s3_key_staging: &str,
    ) -> Result<(), AppError> {
        let media_key = s3_key_staging.replace("staging/", "media/");
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE app.media
            SET s3_key_media = $1, guardduty_scan_status = 'no_threats_found', is_promoted = true, updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(&media_key)
        .bind(&now)
        .bind(&media_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // TODO: S3 copy operation and quota update
        Ok(())
    }

    async fn quarantine_media(
        pool: &PgPool,
        media_id: Uuid,
        _s3_key_staging: &str,
    ) -> Result<(), AppError> {
        let now = Utc::now();

        sqlx::query(
            "UPDATE app.media SET guardduty_scan_status = 'threats_found', updated_at = $1 WHERE id = $2",
        )
        .bind(&now)
        .bind(&media_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // TODO: Alert admin, audit log
        Ok(())
    }

    async fn hold_media(
        pool: &PgPool,
        media_id: Uuid,
        status: &str,
    ) -> Result<(), AppError> {
        let now = Utc::now();

        sqlx::query(
            "UPDATE app.media SET guardduty_scan_status = $1, updated_at = $2 WHERE id = $3",
        )
        .bind(status)
        .bind(&now)
        .bind(&media_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // TODO: Alert and audit
        Ok(())
    }

    /// Get media.
    pub async fn get_media(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        media_id: Uuid,
    ) -> Result<Media, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        sqlx::query_as::<_, Media>("SELECT * FROM app.media WHERE id = $1 AND store_id = $2")
            .bind(&media_id)
            .bind(&store_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::database_error(&e))?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Media not found"))
    }

    /// Get quota.
    pub async fn get_quota(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
    ) -> Result<MediaQuota, AppError> {
        // Verify store ownership
        let _: (Uuid,) = sqlx::query_as(
            "SELECT id FROM app.store WHERE id = $1 AND account_id = $2 AND deleted_at IS NULL",
        )
        .bind(&store_id)
        .bind(&account_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Store not found"))?;

        let quota: Option<MediaQuota> = sqlx::query_as(
            "SELECT store_id, quota_bytes, used_bytes, updated_at FROM app.media_quota WHERE store_id = $1",
        )
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(quota.unwrap_or(MediaQuota {
            store_id,
            quota_bytes: Self::DEFAULT_QUOTA_BYTES,
            used_bytes: 0,
            updated_at: Utc::now(),
        }))
    }
}
