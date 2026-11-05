use crate::error::AppError;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use rand::Rng;

use super::models::*;

pub struct PuckDataManager;

impl PuckDataManager {
    /// Create page
    pub async fn create_page(
        db: &PgPool,
        store_id: Uuid,
        slug: &str,
        title: &str,
    ) -> Result<PuckPage, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO app.puck_page (id, store_id, slug, title, state, created_at, updated_at) VALUES ($1, $2, $3, $4, 'draft', $5, $5)",
        )
        .bind(&id)
        .bind(&store_id)
        .bind(slug)
        .bind(title)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Self::get_page(db, id).await
    }

    /// Get page
    pub async fn get_page(db: &PgPool, page_id: Uuid) -> Result<PuckPage, AppError> {
        sqlx::query_as::<_, PuckPage>(
            "SELECT * FROM app.puck_page WHERE id = $1",
        )
        .bind(&page_id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(crate::error::ErrorCode::NotFound, "Page not found"))
    }

    /// Save page revision
    pub async fn save_revision(
        db: &PgPool,
        page_id: Uuid,
        content: serde_json::Value,
    ) -> Result<PuckPageRevision, AppError> {
        let mut tx = db.begin().await.map_err(|e| AppError::database_error(&e))?;

        // Get next revision number
        let (next_rev,): (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(revision_number), 0) + 1 FROM app.puck_page_revision WHERE page_id = $1",
        )
        .bind(&page_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO app.puck_page_revision (id, page_id, content, revision_number, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(&page_id)
        .bind(&content)
        .bind(next_rev)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Update page updated_at
        sqlx::query("UPDATE app.puck_page SET updated_at = $1 WHERE id = $2")
            .bind(&now)
            .bind(&page_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        Ok(PuckPageRevision {
            id,
            page_id,
            content,
            revision_number: next_rev,
            created_at: now,
        })
    }

    /// Publish page
    pub async fn publish_page(
        db: &PgPool,
        page_id: Uuid,
        revision_id: Uuid,
    ) -> Result<PuckPagePublish, AppError> {
        let mut tx = db.begin().await.map_err(|e| AppError::database_error(&e))?;

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO app.puck_page_publish (id, page_id, revision_id, is_rollback, published_at) VALUES ($1, $2, $3, false, $4)",
        )
        .bind(&id)
        .bind(&page_id)
        .bind(&revision_id)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Update page state
        sqlx::query("UPDATE app.puck_page SET state = 'published', updated_at = $1 WHERE id = $2")
            .bind(&now)
            .bind(&page_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::database_error(&e))?;

        tx.commit().await.map_err(|e| AppError::database_error(&e))?;

        Ok(PuckPagePublish {
            id,
            page_id,
            revision_id,
            is_rollback: false,
            published_at: now,
        })
    }

    /// Generate preview token per D-033 (short-lived, scoped)
    pub async fn generate_preview_token(
        db: &PgPool,
        page_id: Uuid,
    ) -> Result<String, AppError> {
        let mut rng = rand::thread_rng();
        let token: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'a' + (idx - 10)) as char
                }
            })
            .collect();

        let id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(1);

        sqlx::query(
            "INSERT INTO app.page_preview_token (id, page_id, token, expires_at, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(&page_id)
        .bind(&token)
        .bind(&expires_at)
        .bind(&now)
        .execute(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(token)
    }
}
