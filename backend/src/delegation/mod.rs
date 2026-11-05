// backend/src/delegation/mod.rs

//! Delegation module for dual-control approval workflows per D-026.

pub mod approval_request;
pub mod dual_control;
pub mod execution_token;
pub mod models;
pub mod routes;
#[cfg(test)]
pub mod tests;

pub use approval_request::ApprovalRequestManager;
pub use execution_token::ExecutionTokenManager;
