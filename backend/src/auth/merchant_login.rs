// backend/src/auth/merchant_login.rs

//! Merchant login flow per D-012/D-048.

use crate::error::AppResult;

/// POST /api/v1/merchant/login
pub async fn merchant_login() -> AppResult<()> {
    // TODO: Implement merchant login
    Err(crate::error::AppError::NotFound("Not implemented".to_string()))
}
