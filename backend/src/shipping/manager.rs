use crate::error::{AppError, ErrorCode};
use crate::audit::AuditLogger;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use super::models::*;

pub struct ShippingManager;

impl ShippingManager {
    /// Create shipping rule. Branch-specific rules override store-wide rules.
    pub async fn create_rule(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        req: CreateShippingRuleRequest,
    ) -> Result<ShippingRule, AppError> {
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

        // Verify branch if specified
        if let Some(branch_id) = req.branch_id {
            let _: (Uuid,) = sqlx::query_as(
                "SELECT id FROM app.branch WHERE id = $1 AND store_id = $2",
            )
            .bind(&branch_id)
            .bind(&store_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::database_error(&e))?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Branch not found"))?;
        }

        if req.cost_egp < 0 {
            return Err(AppError::new(
                ErrorCode::InvalidInput,
                "Shipping cost cannot be negative",
            ));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO app.shipping_rule (id, store_id, branch_id, governorate, county, district, cost_egp, estimated_days, label, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(&id)
        .bind(&store_id)
        .bind(&req.branch_id)
        .bind(&req.governorate)
        .bind(&req.county)
        .bind(&req.district)
        .bind(req.cost_egp)
        .bind(req.estimated_days)
        .bind(&req.label)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        AuditLogger::log_action(
            pool,
            account_id,
            "shipping_rule.create",
            Some(id),
            serde_json::json!(&req),
            None,
        )
        .await?;

        Self::get_rule(pool, account_id, store_id, id).await
    }

    /// Get rule.
    pub async fn get_rule(
        pool: &PgPool,
        account_id: Uuid,
        store_id: Uuid,
        rule_id: Uuid,
    ) -> Result<ShippingRule, AppError> {
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

        sqlx::query_as::<_, ShippingRule>(
            "SELECT * FROM app.shipping_rule WHERE id = $1 AND store_id = $2",
        )
        .bind(&rule_id)
        .bind(&store_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?
        .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Shipping rule not found"))
    }

    /// Find matching rule per D-119: district > county > governorate specificity.
    pub async fn find_matching_rule(
        pool: &PgPool,
        store_id: Uuid,
        branch_id: Option<Uuid>,
        governorate: &str,
        county: &str,
        district: &str,
    ) -> Result<Option<ShippingRule>, AppError> {
        // Check district-level rules first
        let rule: Option<ShippingRule> = sqlx::query_as(
            r#"
            SELECT * FROM app.shipping_rule
            WHERE store_id = $1 AND district = $2
            AND (branch_id = $3 OR branch_id IS NULL)
            ORDER BY branch_id DESC, cost_egp ASC
            LIMIT 1
            "#,
        )
        .bind(&store_id)
        .bind(district)
        .bind(&branch_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if rule.is_some() {
            return Ok(rule);
        }

        // Check county-level
        let rule: Option<ShippingRule> = sqlx::query_as(
            r#"
            SELECT * FROM app.shipping_rule
            WHERE store_id = $1 AND county = $2 AND district IS NULL
            AND (branch_id = $3 OR branch_id IS NULL)
            ORDER BY branch_id DESC, cost_egp ASC
            LIMIT 1
            "#,
        )
        .bind(&store_id)
        .bind(county)
        .bind(&branch_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        if rule.is_some() {
            return Ok(rule);
        }

        // Check governorate-level
        sqlx::query_as::<_, ShippingRule>(
            r#"
            SELECT * FROM app.shipping_rule
            WHERE store_id = $1 AND governorate = $2 AND county IS NULL AND district IS NULL
            AND (branch_id = $3 OR branch_id IS NULL)
            ORDER BY branch_id DESC, cost_egp ASC
            LIMIT 1
            "#,
        )
        .bind(&store_id)
        .bind(governorate)
        .bind(&branch_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::database_error(&e))
    }
}
