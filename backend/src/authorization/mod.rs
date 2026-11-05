// backend/src/authorization/mod.rs
// REVISED IN WAVE 3b: Added models module

pub mod capability;
pub mod check;
pub mod context;
pub mod middleware;
pub mod models;
#[cfg(test)]
pub mod tests;

pub use capability::CapabilityChecker;
pub use check::AuthorizationCheck;
pub use context::ContextManager;
