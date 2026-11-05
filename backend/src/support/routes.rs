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

use super::manager::SupportManager;
use super::models::*;

pub fn support_routes() -> Router<AppState> {
    Router::new()
        .route("/stores/:store_id/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:ticket_id", get(get_ticket))
        .route("/tickets/:ticket_id/reply", post(reply_ticket))
}

async fn create_ticket(
    State(state): State<AppState>,
    SessionClaim { account_id: consumer_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<TicketResponse>), AppError> {
    let ticket = SupportManager::create_ticket(&state.db, consumer_id, Some(store_id), req).await?;

    Ok((
        StatusCode::CREATED,
        Json(TicketResponse {
            id: ticket.id.to_string(),
            subject: ticket.subject,
            state: ticket.state,
            created_at: ticket.created_at.to_rfc3339(),
        }),
    ))
}

async fn get_ticket(
    State(state): State<AppState>,
    SessionClaim { account_id: consumer_id, .. }: SessionClaim,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<Ticket>, AppError> {
    let ticket = SupportManager::get_ticket(&state.db, ticket_id).await?;
    // Consumers may read only their own tickets. Foreign and missing tickets both return 404.
    if ticket.consumer_id != consumer_id {
        return Err(crate::error::AppError::new(crate::error::ErrorCode::NotFound, "Ticket not found"));
    }
    Ok(Json(ticket))
}

async fn list_tickets(
    State(state): State<AppState>,
    SessionClaim { account_id: consumer_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<Ticket>>, AppError> {
    let tickets = SupportManager::list_consumer_tickets(&state.db, store_id, consumer_id).await?;
    Ok(Json(tickets))
}

async fn reply_ticket(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(ticket_id): Path<Uuid>,
    Json(req): Json<ReplyToTicketRequest>,
) -> Result<(StatusCode, Json<TicketReply>), AppError> {
    let reply = SupportManager::reply_to_ticket(&state.db, ticket_id, account_id, true, req).await?;
    Ok((StatusCode::CREATED, Json(reply)))
}
