// backend/src/lib.rs

//! Rabita commerce operating platform backend.
//!
//! This library exports the core application modules, state, and router setup.

pub mod auth;
pub mod authorization;
pub mod config;
pub mod db;
pub mod delegation;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod state;

pub use error::{AppError, AppResult};
pub use state::AppState;
