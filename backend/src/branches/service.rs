// backend/src/branches/service.rs

//! Branch service per D-062.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct BranchManager;

impl BranchManager {
    /// Create branch per D-062.
    pub async fn create_branch(
        db: &PgPool,
        store_id: Uuid,
        name: &str,
        address: Option<&str>,
        phone_number: Option<&str>,
        pickup_enabled: bool,
    ) -> AppResult<Uuid> {
        let branch_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO app.branch (store_id, name, address, phone_number, pickup_enabled)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(store_id)
        .bind(name)
        .bind(address)
        .bind(phone_number)
        .bind(pickup_enabled)
        .fetch_one(db)
        .await?;

        Ok(branch_id)
    }

    /// Set working hours per D-062.
    pub async fn set_working_hours(
        db: &PgPool,
        branch_id: Uuid,
        day_of_week: i16,
        opens_at: &str,
        closes_at: &str,
    ) -> AppResult<()> {
        // Parse times (HH:MM format)
        let open_time: chrono::NaiveTime = chrono::NaiveTime::parse_from_str(opens_at, "%H:%M")
            .map_err(|_| AppError::BadRequest("Invalid time format".to_string()))?;

        let close_time: chrono::NaiveTime = chrono::NaiveTime::parse_from_str(closes_at, "%H:%M")
            .map_err(|_| AppError::BadRequest("Invalid time format".to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO app.branch_working_hours (branch_id, day_of_week, opens_at, closes_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (branch_id, day_of_week) DO UPDATE
            SET opens_at = $3, closes_at = $4
            "#,
        )
        .bind(branch_id)
        .bind(day_of_week)
        .bind(open_time)
        .bind(close_time)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Get branch by ID.
    pub async fn get_branch(db: &PgPool, branch_id: Uuid) -> AppResult<BranchRow> {
        sqlx::query_as::<_, BranchRow>(
            r#"
            SELECT id, store_id, name, address, phone_number, is_published, pickup_enabled, created_at, updated_at
            FROM app.branch
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(branch_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Branch not found".to_string()))
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct BranchRow {
    pub id: Uuid,
    pub store_id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub is_published: bool,
    pub pickup_enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
