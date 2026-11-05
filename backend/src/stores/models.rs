use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Store {
    pub id: Uuid,
    pub account_id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub is_published: bool,
    pub is_suspended: bool,
    pub suspension_reason: Option<String>,
    pub contact_email: String,
    pub contact_phone: String,
    pub logo_media_id: Option<Uuid>,
    pub favicon_media_id: Option<Uuid>,
    pub accent_color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StoreSettings {
    pub store_id: Uuid,
    pub ticket_lifetime_days: i32, // 10-90, default 14
    pub tax_rate_bp: i32, // basis points, default 1400 (14%)
    pub cancellation_window_seconds: i64, // 0 = disabled, >0 = window in seconds
    pub allow_guest_checkout: bool,
    pub guest_checkout_auto_disable_reason: Option<String>,
    pub max_store_count: i32, // per merchant (may differ from global)
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReservedSlug {
    pub id: Uuid,
    pub slug: String,
    pub tier: String, // "tier1" (immutable engineering) or "tier2" (admin-managed)
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>, // admin who reserved, NULL for tier1
}

#[derive(Debug, Deserialize)]
pub struct CreateStoreRequest {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub contact_email: String,
    pub contact_phone: String,
}

#[derive(Debug, Deserialize)]
#[derive(serde::Serialize)]
pub struct UpdateStoreRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub accent_color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[derive(serde::Serialize)]
pub struct UpdateStoreSettingsRequest {
    pub ticket_lifetime_days: Option<i32>,
    pub tax_rate_bp: Option<i32>,
    pub cancellation_window_seconds: Option<i64>,
    pub allow_guest_checkout: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StoreResponse {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub is_published: bool,
    pub is_suspended: bool,
    pub contact_email: String,
    pub contact_phone: String,
    pub created_at: String,
    pub settings: StoreSettings,
}

#[derive(Debug, Deserialize)]
#[derive(serde::Serialize)]
pub struct SuspendStoreRequest {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct PublishStoreRequest {
    pub publish: bool,
}
