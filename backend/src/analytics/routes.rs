use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::SessionClaim;
use crate::error::AppError;
use crate::state::AppState;

use super::manager::AnalyticsManager;
use super::models::*;

#[derive(Debug, Deserialize)]
pub struct MetricQuery {
    pub metric: String,
    pub days: Option<i32>,
}

pub fn analytics_routes() -> Router<AppState> {
    Router::new()
        .route("/stores/:store_id/analytics/dashboard", get(get_dashboard))
        .route("/stores/:store_id/analytics/metric", get(get_metric))
}

async fn get_dashboard(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Query(params): Query<MetricQuery>,
) -> Result<Json<DashboardMetricsResponse>, AppError> {
    // TODO: Verify store ownership
    let summary = AnalyticsManager::get_dashboard_summary(
        &state.db,
        store_id,
        params.days.unwrap_or(30),
    )
    .await?;

    Ok(Json(summary))
}

async fn get_metric(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Query(params): Query<MetricQuery>,
) -> Result<Json<MetricResponse>, AppError> {
    // TODO: Verify store ownership
    let metric = AnalyticsManager::get_metric(
        &state.db,
        store_id,
        &params.metric,
        params.days.unwrap_or(30),
    )
    .await?;

    Ok(Json(metric))
}
