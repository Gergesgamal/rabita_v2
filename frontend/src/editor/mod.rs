// frontend/src/editor/mod.rs

pub mod config;
pub mod components;
pub mod routes;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuckPage {
    pub id: String,
    pub store_id: String,
    pub slug: String,
    pub content: serde_json::Value,
    pub status: PageStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PageStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "archived")]
    Archived,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuckComponent {
    pub id: String,
    pub name: String,
    pub approval_state: ApprovalState,
    pub source_code: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApprovalState {
    #[serde(rename = "pending_approval")]
    PendingApproval,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "rejected")]
    Rejected,
}
