// backend/src/delegation/routes.rs

//! Delegation routes for approval and execution.

use crate::{
    authorization::AuthorizationCheck,
    auth::session::SessionClaim,
    delegation::models::{ApprovalDecision, ApprovalRequestInput, ExecutionRequest},
    error::AppResult,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub fn router() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/approval-requests", axum::routing::post(create_approval_request))
        .route("/approval-requests/:id/approve", axum::routing::post(approve_request))
        .route("/approval-requests/:id/reject", axum::routing::post(reject_request))
        .route("/execute", axum::routing::post(execute_action))
}

/// POST /api/v1/delegation/approval-requests
async fn create_approval_request(
    State(state): State<AppState>,
    claim: SessionClaim,
    Json(input): Json<ApprovalRequestInput>,
) -> AppResult<(StatusCode, Json<Uuid>)> {
    // TODO: Verify action_type is dual-control capability
    // TODO: Create approval request
    // TODO: Audit

    Err(crate::error::AppError::NotFound(
        "Not implemented".to_string(),
    ))
}

/// POST /api/v1/delegation/approval-requests/:id/approve
async fn approve_request(
    State(state): State<AppState>,
    claim: SessionClaim,
    Path(request_id): Path<Uuid>,
    Json(input): Json<ApprovalDecision>,
) -> AppResult<StatusCode> {
    // TODO: Verify approval permissions
    // TODO: Approve request
    // TODO: Create execution token
    // TODO: Audit

    Err(crate::error::AppError::NotFound(
        "Not implemented".to_string(),
    ))
}

/// POST /api/v1/delegation/approval-requests/:id/reject
async fn reject_request(
    State(state): State<AppState>,
    claim: SessionClaim,
    Path(request_id): Path<Uuid>,
    Json(input): Json<ApprovalDecision>,
) -> AppResult<StatusCode> {
    // TODO: Verify rejection permissions
    // TODO: Reject request
    // TODO: Audit

    Err(crate::error::AppError::NotFound(
        "Not implemented".to_string(),
    ))
}

/// POST /api/v1/delegation/execute
async fn execute_action(
    State(state): State<AppState>,
    claim: SessionClaim,
    Json(request): Json<ExecutionRequest>,
) -> AppResult<StatusCode> {
    // TODO: Verify execution token
    // TODO: Check payload hash matches
    // TODO: Execute action
    // TODO: Audit
    // TODO: Return 409 Conflict if payload mutated

    Err(crate::error::AppError::NotFound(
        "Not implemented".to_string(),
    ))
}
