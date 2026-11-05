use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct MetricValue {
    pub date: String,
    pub value: i64,
}

#[derive(Debug, Serialize)]
pub struct MetricResponse {
    pub metric_name: String,
    pub values: Vec<MetricValue>,
    pub current_period_total: i64,
}

#[derive(Debug, Serialize)]
pub struct TopProduct {
    pub product_id: String,
    pub product_name: String,
    pub units_sold: i64,
    pub revenue_egp: i64,
}

#[derive(Debug, Serialize)]
pub struct DashboardMetricsResponse {
    pub total_views: i64,
    pub total_orders: i64,
    pub total_revenue_egp: i64,
    pub average_order_value_egp: i64,
    pub conversion_rate: f64,
    pub top_products: Vec<TopProduct>,
    pub period: String,
}
