use axum::{
    http::StatusCode,
    routing::get,
    Router,
};
use leptos::*;
use leptos_axum::LeptosRoutes;

mod app;
mod components;
mod error;
mod i18n;
mod pages;
mod server;
mod utils;

use app::App;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .fallback(default_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn ready() -> StatusCode {
    StatusCode::OK
}

async fn default_handler() -> StatusCode {
    StatusCode::NOT_FOUND
}
