// backend/src/delegation/models.rs

//! Delegation models.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequestInput {
    pub action_type: String, // 'employees.manage', 'orders.refund', etc.
    pub action_payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    pub decision: String, // 'approved' or 'rejected'
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub approval_request_id: Uuid,
    pub execution_token: String,
    pub action_payload: serde_json::Value,
}
