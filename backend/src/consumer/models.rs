use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConsumerAccount {
    pub id: Uuid,
    pub store_id: Uuid,
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConsumerAddress {
    pub id: Uuid,
    pub consumer_id: Uuid,
    pub governorate: String,
    pub county: String,
    pub district: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConsumerConsent {
    pub consumer_id: Uuid,
    pub marketing_emails: bool,
    pub whatsapp_messages: bool,
    pub analytics: bool,
    pub third_party: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddAddressRequest {
    pub governorate: String,
    pub county: String,
    pub district: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConsentRequest {
    pub marketing_emails: Option<bool>,
    pub whatsapp_messages: Option<bool>,
    pub analytics: Option<bool>,
    pub third_party: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ConsumerProfileResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub addresses: Vec<ConsumerAddress>,
    pub consent: ConsumerConsent,
}
