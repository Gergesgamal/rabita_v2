// backend/src/auth/errors.rs

//! Authentication-specific error handling with uniform timing per D-012.

use crate::error::AppError;
use std::time::{Duration, Instant};

/// Uniform timing for authentication failures per D-012.
pub async fn uniform_auth_delay() {
    // Constant delay to prevent timing attacks
    tokio::time::sleep(Duration::from_millis(100)).await;
}

/// Log authentication failure with protocol violation detection per D-012.
pub fn log_auth_failure(
    event_type: &str,
    ip_address: Option<&str>,
    account_id: Option<&str>,
    is_protocol_violation: bool,
) {
    let event_level = if is_protocol_violation {
        "protocol_violation"
    } else {
        "auth_failure"
    };
    
    tracing::warn!(
        event = event_level,
        event_type = event_type,
        ip_address = ip_address,
        account_id = account_id,
        "Authentication event"
    );
}

/// Return generic auth failure response.
pub fn generic_auth_error() -> AppError {
    AppError::AuthenticationFailed
}
