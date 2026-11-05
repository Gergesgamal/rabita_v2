// frontend/src/error.rs

use leptos::*;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[component]
pub fn ErrorBoundary(children: Children) -> impl IntoView {
    view! {
        <div class="error-boundary">
            {children()}
        </div>
    }
}
