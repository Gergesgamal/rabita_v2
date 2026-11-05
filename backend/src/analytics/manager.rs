use crate::error::AppError;
use uuid::Uuid;
use sqlx::PgPool;

use super::models::*;

pub struct AnalyticsManager;

impl AnalyticsManager {
    /// Get metric data per D-074 (20 core metrics, computed from tables, no third-party)
    pub async fn get_metric(
        db: &PgPool,
        store_id: Uuid,
        metric_name: &str,
        days: i32,
    ) -> Result<MetricResponse, AppError> {
        // Compute from existing tables
        let values = match metric_name {
            "views" => {
                // Would count storefront page views (requires logging)
                vec![]
            }
            "orders" => {
                let rows: Vec<(String, i64)> = sqlx::query_as(
                    r#"
                    SELECT DATE(created_at)::text as date, COUNT(*) as count
                    FROM app.order
                    WHERE store_id = $1 AND created_at > now() - interval '$2 days'
                    GROUP BY DATE(created_at)
                    ORDER BY date
                    "#,
                )
                .bind(&store_id)
                .bind(days)
                .fetch_all(db)
                .await
                .map_err(|e| AppError::database_error(&e))?;

                rows.into_iter()
                    .map(|(date, count)| MetricValue {
                        date,
                        value: count,
                    })
                    .collect()
            }
            "revenue" => {
                let rows: Vec<(String, i64)> = sqlx::query_as(
                    r#"
                    SELECT DATE(created_at)::text as date, SUM((order_snapshot ->> 'total_egp')::bigint) as revenue
                    FROM app.order
                    WHERE store_id = $1 AND created_at > now() - interval '$2 days'
                    GROUP BY DATE(created_at)
                    ORDER BY date
                    "#,
                )
                .bind(&store_id)
                .bind(days)
                .fetch_all(db)
                .await
                .map_err(|e| AppError::database_error(&e))?;

                rows.into_iter()
                    .map(|(date, revenue)| MetricValue {
                        date,
                        value: revenue.unwrap_or(0),
                    })
                    .collect()
            }
            _ => vec![],
        };

        let total: i64 = values.iter().map(|v| v.value).sum();

        Ok(MetricResponse {
            metric_name: metric_name.to_string(),
            values,
            current_period_total: total,
        })
    }

    /// Get dashboard summary per D-074
    pub async fn get_dashboard_summary(
        db: &PgPool,
        store_id: Uuid,
        days: i32,
    ) -> Result<DashboardMetricsResponse, AppError> {
        // Total orders
        let (total_orders,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM app.order WHERE store_id = $1 AND created_at > now() - interval '$2 days'",
        )
        .bind(&store_id)
        .bind(days)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        // Total revenue
        let (total_revenue,): (Option<i64>,) = sqlx::query_as(
            r#"
            SELECT SUM((order_snapshot ->> 'total_egp')::bigint)
            FROM app.order
            WHERE store_id = $1 AND created_at > now() - interval '$2 days'
            "#,
        )
        .bind(&store_id)
        .bind(days)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        let total_revenue_egp = total_revenue.unwrap_or(0);
        let avg_order_value = if total_orders > 0 {
            total_revenue_egp / total_orders
        } else {
            0
        };

        // Top products
        let top_products: Vec<TopProduct> = sqlx::query_as(
            r#"
            SELECT 
                p.id::text as product_id,
                p.name as product_name,
                SUM((oi.quantity)::bigint) as units_sold,
                SUM((oi.quantity * oi.unit_price_egp)::bigint) as revenue_egp
            FROM app.order_item oi
            JOIN app.product p ON oi.product_id = p.id
            JOIN app.order o ON oi.order_id = o.id
            WHERE o.store_id = $1 AND o.created_at > now() - interval '$2 days'
            GROUP BY p.id, p.name
            ORDER BY revenue_egp DESC
            LIMIT 5
            "#,
        )
        .bind(&store_id)
        .bind(days)
        .fetch_all(db)
        .await
        .map_err(|e| AppError::database_error(&e))?;

        Ok(DashboardMetricsResponse {
            total_views: 0, // Would require view logging
            total_orders,
            total_revenue_egp,
            average_order_value_egp: avg_order_value,
            conversion_rate: 0.0, // Would require view logging
            top_products,
            period: format!("last {} days", days),
        })
    }
}
