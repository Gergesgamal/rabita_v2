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

use super::manager::JobsManager;
use super::models::*;

pub fn jobs_routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", get(list_public_jobs))
        .route("/stores/:store_id/jobs", post(create_job).get(list_store_jobs))
        .route("/jobs/:job_id", get(get_job))
        .route("/jobs/:job_id/apply", post(apply_to_job))
        .route("/jobs/applications/:app_id/chat", post(send_message))
}

async fn list_public_jobs(
    State(state): State<AppState>,
) -> Result<Json<Vec<JobPost>>, AppError> {
    let jobs = JobsManager::list_public_jobs(&state.db).await?;
    Ok(Json(jobs))
}

async fn create_job(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
    Json(req): Json<CreateJobPostRequest>,
) -> Result<(StatusCode, Json<JobPostResponse>), AppError> {
    // TODO: Verify store ownership
    let job = JobsManager::create_job_post(&state.db, store_id, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(JobPostResponse {
            id: job.id.to_string(),
            title: job.title,
            state: job.state,
            created_at: job.created_at.to_rfc3339(),
        }),
    ))
}

async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<JobPost>, AppError> {
    let job = JobsManager::get_job_post(&state.db, job_id).await?;
    Ok(Json(job))
}

async fn list_store_jobs(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<JobPost>>, AppError> {
    // TODO: Verify store ownership
    let jobs: Vec<JobPost> = sqlx::query_as(
        "SELECT * FROM app.job_post WHERE store_id = $1 ORDER BY created_at DESC",
    )
    .bind(&store_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    Ok(Json(jobs))
}

async fn apply_to_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
    Json(req): Json<SubmitApplicationRequest>,
) -> Result<(StatusCode, Json<JobApplication>), AppError> {
    let app = JobsManager::apply_to_job(&state.db, job_id, req).await?;
    Ok((StatusCode::CREATED, Json(app)))
}

async fn send_message(
    State(state): State<AppState>,
    SessionClaim { account_id, .. }: SessionClaim,
    Path(app_id): Path<Uuid>,
    Json(req): Json<SendChatMessageRequest>,
) -> Result<(StatusCode, Json<JobChatMessage>), AppError> {
    // Get thread ID from application
    let thread: (Uuid,) = sqlx::query_as(
        "SELECT thread_id FROM app.job_chat_thread WHERE application_id = $1",
    )
    .bind(&app_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::database_error(&e))?;

    let msg = JobsManager::send_message(&state.db, thread.0, account_id, true, req).await?;
    Ok((StatusCode::CREATED, Json(msg)))
}
