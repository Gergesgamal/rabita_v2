// backend/src/consumer_auth/mod.rs

//! Consumer authentication: signup per D-131, email verification, WhatsApp OTP per D-128, password reset.

pub mod login;
pub mod reset;
pub mod signup;
pub mod routes;
#[cfg(test)]
pub mod tests;
pub mod models;

pub use signup::ConsumerSignupManager;
pub use login::ConsumerLoginManager;
pub use reset::ConsumerPasswordResetManager;
