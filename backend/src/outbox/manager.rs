use crate::error::AppError;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct OutboxManager;

impl OutboxManager {
    const MAX_RETRIES: i32 = 5;
    const LEASE_DURATION_SECONDS: i64 = 60;
    const INITIAL_BACKOFF_MS: i64 = 100;

    /// Enqueue event per D-084
    pub async fn enqueue_event(
        db: &PgPool,
        event_type: &str,
        aggregate_id: Uuid,
        payload: serde_json::Value,
    ) -> Result<Uuid, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.outbox (id, event_type, aggregate_id, payload, state, attempt_count, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'pending', 0, $5, $5)
            "#,
        )
        .bind(&id)
        .bind(event_type)
        .bind(&aggregate_id)
        .bind(&payload)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(id)
    }

    /// Claim pending events with lease per D-084
    pub async fn claim_events(
        db: &PgPool,
        worker_id: &str,
        batch_size: i32,
    ) -> Result<Vec<OutboxEvent>, AppError> {
        let now = Utc::now();
        let lease_until = now + chrono::Duration::seconds(Self::LEASE_DURATION_SECONDS);

        // Claim expired and new leases
        let events: Vec<OutboxEvent> = sqlx::query_as(
            r#"
            SELECT * FROM app.outbox
            WHERE state = 'pending'
            AND (lease_until IS NULL OR lease_until < now())
            ORDER BY created_at ASC
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(batch_size)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Update lease
        for event in &events {
            sqlx::query(
                "UPDATE app.outbox SET state = 'processing', lease_until = $1, updated_at = $2 WHERE id = $3",
            )
            .bind(&lease_until)
            .bind(&now)
            .bind(&event.id)
            .execute(db)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        }

        Ok(events)
    }

    /// Mark event as completed
    pub async fn mark_completed(db: &PgPool, event_id: Uuid) -> Result<(), AppError> {
        let now = Utc::now();

        sqlx::query(
            "UPDATE app.outbox SET state = 'completed', lease_until = NULL, updated_at = $1 WHERE id = $2",
        )
        .bind(&now)
        .bind(&event_id)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(())
    }

    /// Mark event as failed with retry or DLQ
    pub async fn mark_failed(
        db: &PgPool,
        event_id: Uuid,
        error_message: &str,
    ) -> Result<(), AppError> {
        let mut tx = db.begin().await.map_err(|e| AppError::database_error(&e))?;

        let event: (i32,) = sqlx::query_as(
            "SELECT attempt_count FROM app.outbox WHERE id = $1",
        )
        .bind(&event_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let now = Utc::now();

        // Log attempt
        sqlx::query(
            "INSERT INTO app.outbox_attempt (id, event_id, attempt_number, status, error_message, created_at) VALUES ($1, $2, $3, 'failure', $4, $5)",
        )
        .bind(Uuid::new_v4())
        .bind(&event_id)
        .bind(event.0 + 1)
        .bind(error_message)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if event.0 >= Self::MAX_RETRIES {
            // Move to DLQ
            sqlx::query(
                "UPDATE app.outbox SET state = 'dlq', lease_until = NULL, attempt_count = $1, last_error = $2, updated_at = $3 WHERE id = $4",
            )
            .bind(event.0 + 1)
            .bind(error_message)
            .bind(&now)
            .bind(&event_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        } else {
            // Retry with backoff
            let backoff_ms = Self::INITIAL_BACKOFF_MS * 2i64.pow(event.0 as u32);
            let lease_until = now + chrono::Duration::milliseconds(backoff_ms);

            sqlx::query(
                "UPDATE app.outbox SET state = 'pending', lease_until = $1, attempt_count = $2, last_error = $3, updated_at = $4 WHERE id = $5",
            )
            .bind(&lease_until)
            .bind(event.0 + 1)
            .bind(error_message)
            .bind(&now)
            .bind(&event_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;
        }

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        Ok(())
    }
}
