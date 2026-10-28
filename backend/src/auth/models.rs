// backend/src/auth/models.rs

//! Authentication data types and models.

use serde::{Deserialize, Serialize};

/// Placeholder for Wave 3 auth models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPlaceholder {
    pub note: String,
}
