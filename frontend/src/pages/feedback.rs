use leptos::*;
use crate::components::feedback_form::FeedbackForm;

#[component]
pub fn FeedbackPage() -> impl IntoView {
    view! {
        <div class="feedback-page">
            <FeedbackForm />
        </div>
    }
}
