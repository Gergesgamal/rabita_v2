//! Shared application error type. Every error response is RFC 9457 Problem Details
//! (application/problem+json). Server-side causes are logged and never sent to clients.

use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

const GENERIC_SERVER_DETAIL: &str = "An internal error occurred.";

pub type AppResult<T> = Result<T, AppError>;

/// Machine-readable categories for `AppError::new`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    NotFound,
    InvalidInput,
    InvalidState,
    Conflict,
    RateLimited,
    QuotaExceeded,
    CacheError,
    InternalError,
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized,
    AuthenticationFailed,
    InvalidToken,
    SessionExpired,
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    Database(String),
    InternalError(String),
    /// A dependency is down. The message must be safe to show to clients.
    DependencyUnavailable(String),
    Coded { code: ErrorCode, message: String },
}

/// What a client may see for one error.
struct PublicError {
    status: StatusCode,
    code: &'static str,
    detail: String,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        AppError::Coded { code, message: message.into() }
    }

    /// Logs the driver error server-side. The client only ever sees a generic message.
    pub fn database_error(err: &sqlx::Error) -> Self {
        tracing::error!(error = %err, "database error");
        AppError::Database(err.to_string())
    }

    fn public(&self) -> PublicError {
        let (status, code, detail) = match self {
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, "invalid_input", m.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", "Authentication required.".into()),
            AppError::AuthenticationFailed => (StatusCode::UNAUTHORIZED, "authentication_failed", "Invalid credentials.".into()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid_token", "The token is invalid or expired.".into()),
            AppError::SessionExpired => (StatusCode::UNAUTHORIZED, "session_expired", "The session has expired.".into()),
            AppError::Forbidden(m) => (StatusCode::FORBIDDEN, "forbidden", m.clone()),
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, "not_found", m.clone()),
            AppError::Conflict(m) => (StatusCode::CONFLICT, "conflict", m.clone()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error", GENERIC_SERVER_DETAIL.into()),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", GENERIC_SERVER_DETAIL.into()),
            AppError::DependencyUnavailable(m) => (StatusCode::SERVICE_UNAVAILABLE, "dependency_unavailable", m.clone()),
            AppError::Coded { code, message } => match code {
                ErrorCode::NotFound => (StatusCode::NOT_FOUND, "not_found", message.clone()),
                ErrorCode::InvalidInput => (StatusCode::BAD_REQUEST, "invalid_input", message.clone()),
                ErrorCode::InvalidState => (StatusCode::CONFLICT, "invalid_state", message.clone()),
                ErrorCode::Conflict => (StatusCode::CONFLICT, "conflict", message.clone()),
                ErrorCode::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "rate_limited", message.clone()),
                ErrorCode::QuotaExceeded => (StatusCode::FORBIDDEN, "quota_exceeded", message.clone()),
                ErrorCode::CacheError => (StatusCode::SERVICE_UNAVAILABLE, "cache_unavailable", GENERIC_SERVER_DETAIL.into()),
                ErrorCode::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", GENERIC_SERVER_DETAIL.into()),
            },
        };
        PublicError { status, code, detail }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::database_error(&err)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        tracing::error!(error = %err, "cache error");
        AppError::new(ErrorCode::CacheError, "cache unavailable")
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

/// RFC 9457 problem document. `type` is "about:blank" (RFC 9457 section 4.2.1: no extra
/// semantics beyond the status). `code` is an extension member for machine handling.
#[derive(Serialize)]
struct Problem<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
    title: &'a str,
    status: u16,
    detail: &'a str,
    code: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let public = self.public();
        if public.status.is_server_error() {
            tracing::error!(error = ?self, status = %public.status, "request failed");
        }

        let title = public.status.canonical_reason().unwrap_or("Error");
        let body = Problem {
            kind: "about:blank",
            title,
            status: public.status.as_u16(),
            detail: &public.detail,
            code: public.code,
        };
        let bytes = serde_json::to_vec(&body)
            .unwrap_or_else(|_| br#"{"title":"Internal Server Error","status":500}"#.to_vec());

        let mut response = (
            public.status,
            [(header::CONTENT_TYPE, "application/problem+json")],
            bytes,
        )
            .into_response();

        // RFC 7235: a 401 must carry a challenge.
        if public.status == StatusCode::UNAUTHORIZED {
            response
                .headers_mut()
                .insert(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn render(err: AppError) -> Response {
        err.into_response()
    }

    async fn body_text(resp: Response) -> String {
        let bytes = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn client_error_is_problem_json() {
        let resp = render(AppError::NotFound("Ticket not found".into())).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        assert_eq!(resp.headers()[header::CONTENT_TYPE], "application/problem+json");
        let body = body_text(resp).await;
        assert!(body.contains(r#""status":404"#));
        assert!(body.contains(r#""code":"not_found""#));
        assert!(body.contains("Ticket not found"));
    }

    #[tokio::test]
    async fn server_error_hides_internal_detail() {
        let resp = render(AppError::Database("relation secret_table does not exist".into())).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = body_text(resp).await;
        assert!(!body.contains("secret_table"));
        assert!(body.contains(GENERIC_SERVER_DETAIL));
    }

    #[tokio::test]
    async fn unauthorized_carries_bearer_challenge() {
        let resp = render(AppError::Unauthorized).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(resp.headers()[header::WWW_AUTHENTICATE], "Bearer");
    }
}
