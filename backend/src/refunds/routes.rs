use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::auth::SessionClaim;
use crate::error::AppError;
use crate::state::AppState;

use super::manager::RefundManager;
use super::models::*;

pub fn refund_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/stores/:store_id/orders/:order_id/refund-request",
            post(request_refund),
        )
        .route(
            "/stores/:store_id/refunds/:refund_id",
            get(get_refund),
        )
        .route(
            "/stores/:store_id/refunds/:refund_id/decide",
            post(decide_refund),
        )
        .route(
            "/stores/:store_id/refunds",
            get(list_refunds),
        )
}

async fn request_refund(
    State(state): State<AppState>,
    SessionClaim { consumer_id, .. }: SessionClaim,
    Path((store_id, order_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<CreateRefundRequestReq>,
) -> Result<(StatusCode, Json<RefundResponse>), AppError> {
    let refund = RefundManager::request_refund(&state.db, store_id, order_id, consumer_id, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(RefundResponse {
            id: refund.id.to_string(),
            state: refund.state,
            refund_amount_egp: refund.refund_amount_egp,
            reason: refund.reason,
            created_at: refund.created_at.to_rfc3339(),
        }),
    ))
}

async fn get_refund(
    State(state): State<AppState>,
    Path((store_id, refund_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<RefundRequest>, AppError> {
    let refund = RefundManager::get_refund(&state.db, store_id, refund_id).await?;
    Ok(Json(refund))
}

async fn decide_refund(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path((store_id, refund_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<ApproveRefundRequest>,
) -> Result<Json<RefundRequest>, AppError> {
    let refund = RefundManager::merchant_decide_refund(
        &state.db,
        store_id,
        account_id,
        refund_id,
        req.approved,
        req.reason.as_deref(),
    )
    .await?;

    Ok(Json(refund))
}

async fn list_refunds(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<RefundRequest>>, AppError> {
    // TODO: Verify store ownership
    let refunds = RefundManager::list_refunds(&state.db, store_id).await?;
    Ok(Json(refunds))
}
