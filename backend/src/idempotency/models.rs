//! Principal types for idempotency keys (D-130). The namespaces never collide.

use crate::error::{AppError, ErrorCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrincipalType {
    Account,
    Consumer,
    GuestSession,
    Service,
}

impl PrincipalType {
    pub fn as_str(self) -> &'static str {
        match self {
            PrincipalType::Account => "account",
            PrincipalType::Consumer => "consumer",
            PrincipalType::GuestSession => "guest_session",
            PrincipalType::Service => "service",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AppError> {
        match value {
            "account" => Ok(PrincipalType::Account),
            "consumer" => Ok(PrincipalType::Consumer),
            "guest_session" => Ok(PrincipalType::GuestSession),
            "service" => Ok(PrincipalType::Service),
            _ => Err(AppError::new(ErrorCode::InvalidInput, "unknown principal type")),
        }
    }
}
