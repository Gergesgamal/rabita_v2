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

use super::manager::FeedbackManager;
use super::models::*;

pub fn feedback_routes() -> Router<AppState> {
    Router::new()
        .route("/feedback", post(create_feedback).get(list_feedback))
        .route("/feedback/:id", get(get_feedback))
}

async fn create_feedback(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Json(req): Json<CreateFeedbackRequest>,
) -> Result<(StatusCode, Json<FeedbackResponse>), AppError> {
    let feedback = FeedbackManager::create_feedback(&state.db, account_id, None, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(FeedbackResponse {
            id: feedback.id.to_string(),
            category: feedback.category,
            title: feedback.title,
            state: feedback.state,
            created_at: feedback.created_at.to_rfc3339(),
        }),
    ))
}

async fn get_feedback(
    State(state): State<AppState>,
    Path(feedback_id): Path<Uuid>,
) -> Result<Json<FeedbackItem>, AppError> {
    let feedback = FeedbackManager::get_feedback(&state.db, feedback_id).await?;
    Ok(Json(feedback))
}

async fn list_feedback(
    State(state): State<AppState>,
) -> Result<Json<Vec<FeedbackItem>>, AppError> {
    let items = FeedbackManager::list_public_feedback(&state.db).await?;
    Ok(Json(items))
}
