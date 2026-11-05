// backend/src/authorization/models.rs

//! Authorization models.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGrant {
    pub account_id: Uuid,
    pub context_id: Uuid,
    pub capability: String,
    pub granted_by: Uuid,
    pub granted_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSwitch {
    pub new_context_id: Uuid,
}
