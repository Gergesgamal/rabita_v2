// backend/src/error.rs

//! Application error type and RFC 9457 Problem Details serialization.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

/// RFC 9457 Problem Details object.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub type_url: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

/// Application error type.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error")]
    InternalError(#[from] anyhow::Error),

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Session expired")]
    SessionExpired,
}

impl AppError {
    /// Get the HTTP status code for this error.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::AuthenticationFailed => StatusCode::UNAUTHORIZED,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::SessionExpired => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Create a Problem Details response.
    fn problem_details(&self) -> ProblemDetails {
        let status = self.status_code();
        let (title, detail) = match self {
            Self::BadRequest(msg) => ("Bad Request", Some(msg.clone())),
            Self::Unauthorized => ("Unauthorized", None),
            Self::Forbidden(msg) => ("Forbidden", Some(msg.clone())),
            Self::NotFound(msg) => ("Not Found", Some(msg.clone())),
            Self::Conflict(msg) => ("Conflict", Some(msg.clone())),
            Self::AuthenticationFailed => ("Authentication Failed", None),
            Self::InvalidToken => ("Invalid Token", None),
            Self::SessionExpired => ("Session Expired", None),
            _ => ("Internal Server Error", None),
        };

        ProblemDetails {
            type_url: format!("https://rabita.shop/errors/{}", status.as_u16()),
            title: title.to_string(),
            status: status.as_u16(),
            detail,
            instance: None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let problem = self.problem_details();

        tracing::error!(?self, "Request error");

        (status, Json(problem)).into_response()
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        Self::BadRequest(msg)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        Self::BadRequest(msg.to_string())
    }
}
