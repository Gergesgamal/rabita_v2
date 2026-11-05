// frontend/src/merchant/feedback.rs

use leptos::*;

#[component]
pub fn FeedbackPage() -> impl IntoView {
    view! {
        <div class="merchant-feedback-page">
            <h2>"الملاحظات والاقتراحات"</h2>
            <FeedbackItems />
        </div>
    }
}

#[component]
fn FeedbackItems() -> impl IntoView {
    view! {
        <div class="feedback-items">
            <div class="feedback-item">
                <h4>"اقتراح بشأن الواجهة"</h4>
                <p>"قد يكون من الأفضل إضافة فلترة متقدمة"</p>
                <p class="feedback-date">"2026-10-28"</p>
            </div>
        </div>
    }
}
