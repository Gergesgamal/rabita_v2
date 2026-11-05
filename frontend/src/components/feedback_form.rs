use leptos::*;

#[component]
pub fn FeedbackForm() -> impl IntoView {
    let (category, set_category) = create_signal("feature".to_string());
    let (message, set_message) = create_signal(String::new());

    view! {
        <div class="feedback-form">
            <select on:change=move |ev| {
                set_category.set(event_target_value(&ev));
            }>
                <option value="feature">"Feature Request"</option>
                <option value="bug">"Bug Report"</option>
            </select>
            <textarea on:change=move |ev| {
                set_message.set(event_target_value(&ev));
            }></textarea>
        </div>
    }
}
