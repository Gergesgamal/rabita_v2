// backend/src/idempotency/mod.rs

//! Idempotency key store per D-130: principal-scoped (account | consumer | guest_session | service).

pub mod middleware;
pub mod store;
pub mod models;
#[cfg(test)]
pub mod tests;

pub use store::IdempotencyStore;
