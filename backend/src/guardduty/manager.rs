use crate::error::AppError;
use crate::media::MediaManager;
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Payload from EventBridge when GuardDuty scan status tag changes
#[derive(Debug, Deserialize)]
pub struct GuardDutyScanStatusEvent {
    pub s3_bucket: String,
    pub s3_object: String,
    pub scan_status: String, // NO_THREATS_FOUND, THREATS_FOUND, UNSUPPORTED, ACCESS_DENIED, FAILED
    pub timestamp: String,
}

pub struct GuardDutyManager;

impl GuardDutyManager {
    /// Handle incoming EventBridge event for scan status change.
    /// Called by Lambda/worker receiving events from rabita-staging bucket.
    pub async fn handle_scan_event(
        pool: &PgPool,
        event: GuardDutyScanStatusEvent,
    ) -> Result<(), AppError> {
        // Find media record by S3 key
        let media_id: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM app.media WHERE s3_key_staging = $1",
        )
        .bind(&event.s3_object)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let media_id = media_id
            .ok_or_else(|| AppError::new(
                crate::error::ErrorCode::NotFound,
                "Media record not found for S3 key",
            ))?
            .0;

        // Route to appropriate handler based on status
        MediaManager::on_scan_status_changed(pool, media_id, event.s3_object, &event.scan_status)
            .await?;

        Ok(())
    }
}
