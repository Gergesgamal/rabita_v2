// backend/src/onboarding/mod.rs

//! Merchant onboarding per D-039 (invite_only, review, open modes) + D-045 (manual provisioning).

pub mod application;
pub mod provisioning;
pub mod routes;
#[cfg(test)]
pub mod tests;
pub mod models;

pub use application::ApplicationManager;
pub use provisioning::ProvisioningManager;
