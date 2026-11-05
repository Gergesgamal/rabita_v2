use leptos::*;

#[component]
pub fn OrderDetail(#[prop(into)] order_id: String) -> impl IntoView {
    view! {
        <div class="order-detail">
            <div>"Order: " {order_id}</div>
        </div>
    }
}
