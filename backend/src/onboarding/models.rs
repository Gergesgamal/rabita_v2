// backend/src/onboarding/models.rs

//! Onboarding data types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct MerchantApplicationInput {
    pub business_name: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub email: String,
    pub product_category: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MerchantApplicationResponse {
    pub id: Uuid,
    pub state: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProvisioningResponse {
    pub identifier: String,
    pub temp_password: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
