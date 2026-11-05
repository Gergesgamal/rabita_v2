use crate::error::{AppError, ErrorCode};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct JobsManager;

impl JobsManager {
    /// Create job post (merchant only)
    pub async fn create_job_post(
        db: &PgPool,
        store_id: Uuid,
        req: CreateJobPostRequest,
    ) -> Result<JobPost, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.job_post (id, store_id, title, description, category, contact_phone, state, is_confidential, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, 'draft', $7, $8, $8)
            "#,
        )
        .bind(&id)
        .bind(&store_id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(&req.category)
        .bind(&req.contact_phone)
        .bind(req.is_confidential.unwrap_or(false))
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Self::get_job_post(db, id).await
    }

    /// Get job post
    pub async fn get_job_post(db: &PgPool, job_id: Uuid) -> Result<JobPost, AppError> {
        sqlx::query_as::<_, JobPost>(
            "SELECT * FROM app.job_post WHERE id = $1",
        )
        .bind(&job_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Job post not found"))
    }

    /// List public job posts (for public /jobs page)
    pub async fn list_public_jobs(db: &PgPool) -> Result<Vec<JobPost>, AppError> {
        sqlx::query_as::<_, JobPost>(
            "SELECT * FROM app.job_post WHERE state = 'public' ORDER BY created_at DESC",
        )
        .fetch_all(db)
        .await
        .map_err(|e| AppError::database_error(&e))
    }

    /// Apply to job
    pub async fn apply_to_job(
        db: &PgPool,
        job_id: Uuid,
        req: SubmitApplicationRequest,
    ) -> Result<JobApplication, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.job_application (id, job_id, applicant_name, applicant_email, state, cv_media_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'applied', $5, $6, $6)
            "#,
        )
        .bind(&id)
        .bind(&job_id)
        .bind(&req.applicant_name)
        .bind(&req.applicant_email)
        .bind(&req.cv_media_id)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Create chat thread
        let thread_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO app.job_chat_thread (id, job_id, application_id, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(&thread_id)
        .bind(&job_id)
        .bind(&id)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Self::get_application(db, id).await
    }

    /// Get application
    pub async fn get_application(db: &PgPool, app_id: Uuid) -> Result<JobApplication, AppError> {
        sqlx::query_as::<_, JobApplication>(
            "SELECT * FROM app.job_application WHERE id = $1",
        )
        .bind(&app_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Application not found"))
    }

    /// Send chat message (rate limit: 10 per minute per thread)
    pub async fn send_message(
        db: &PgPool,
        thread_id: Uuid,
        sender_id: Uuid,
        is_from_merchant: bool,
        req: SendChatMessageRequest,
    ) -> Result<JobChatMessage, AppError> {
        // Rate limit check
        let recent: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM app.job_chat_message WHERE thread_id = $1 AND created_at > now() - interval '1 minute'",
        )
        .bind(&thread_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if recent.0 >= 10 {
            return Err(AppError::new(
                ErrorCode::RateLimited,
                "Message rate limit exceeded",
            ));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO app.job_chat_message (id, thread_id, sender_id, content, is_from_merchant, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(&id)
        .bind(&thread_id)
        .bind(&sender_id)
        .bind(&req.content)
        .bind(is_from_merchant)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(JobChatMessage {
            id,
            thread_id,
            sender_id,
            content: req.content,
            is_from_merchant,
            created_at: now,
        })
    }
}
