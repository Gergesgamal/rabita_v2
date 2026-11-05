use crate::error::{AppError, ErrorCode};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct FeedbackManager;

impl FeedbackManager {
    /// Create feedback (rate limited per role)
    pub async fn create_feedback(
        db: &PgPool,
        account_id: Uuid,
        store_id: Option<Uuid>,
        req: CreateFeedbackRequest,
    ) -> Result<FeedbackItem, AppError> {
        // Rate limits per D-077
        let rate_limit = match req.category.as_str() {
            "bug" => 5, // Consumer: 5 per day
            _ => match store_id {
                Some(_) => 10, // Merchant: 10 per day
                None => 20,    // Employee: 20 per day
            },
        };

        let recent: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM app.feedback_item WHERE account_id = $1 AND created_at > now() - interval '1 day'",
        )
        .bind(&account_id)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if recent.0 >= rate_limit {
            return Err(AppError::new(
                ErrorCode::RateLimited,
                "Daily feedback limit exceeded",
            ));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        // Bugs are never published per D-077 (CHECK constraint)
        let is_published = req.category != "bug";

        sqlx::query(
            r#"
            INSERT INTO app.feedback_item (id, account_id, store_id, category, title, description, state, is_published, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, 'open', $7, $8, $8)
            "#,
        )
        .bind(&id)
        .bind(&account_id)
        .bind(&store_id)
        .bind(&req.category)
        .bind(&req.title)
        .bind(&req.description)
        .bind(is_published)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Self::get_feedback(db, id).await
    }

    /// Get feedback (visibility rules)
    pub async fn get_feedback(db: &PgPool, feedback_id: Uuid) -> Result<FeedbackItem, AppError> {
        sqlx::query_as::<_, FeedbackItem>(
            "SELECT * FROM app.feedback_item WHERE id = $1",
        )
        .bind(&feedback_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Feedback not found"))
    }

    /// List published feedback (admin can see all)
    pub async fn list_public_feedback(db: &PgPool) -> Result<Vec<FeedbackItem>, AppError> {
        sqlx::query_as::<_, FeedbackItem>(
            "SELECT * FROM app.feedback_item WHERE is_published = true AND category != 'bug' ORDER BY updated_at DESC",
        )
        .fetch_all(db)
        .await
        .map_err(|e| AppError::database_error(&e))
    }
}
