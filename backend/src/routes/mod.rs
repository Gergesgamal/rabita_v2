use axum::{routing::get, Router};

use crate::{
    auth, cart, consumer_auth, delegation, error::AppError, media, onboarding, orders, promotions,
    state::AppState, stores, support,
};

/// Top-level router: all 31 business routes plus health and readiness.
/// Each module owns its paths. A duplicate path+method panics at startup.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .merge(auth::routes::router())                 // 2
        .merge(cart::cart_routes())                    // 3
        .merge(consumer_auth::routes::router())        // 3
        .merge(delegation::routes::router())           // 4
        .merge(media::media_routes())                  // 3
        .merge(onboarding::routes::router())           // 2
        .merge(orders::order_routes())                 // 4
        .merge(promotions::promotion_routes())         // 2
        .merge(stores::store_routes())                 // 5
        .merge(support::support_routes())              // 3
        .fallback(route_not_found)
}

/// Unmatched paths return the same problem document as every other error.
async fn route_not_found() -> AppError {
    AppError::NotFound("No route matches this path.".to_string())
}

pub async fn health() -> &'static str {
    "ok"
}

// Still a placeholder: D-126 requires real Postgres and Valkey checks.
pub async fn ready() -> &'static str {
    "ready"
}
