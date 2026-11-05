use leptos::*;
use leptos_router::*;
use crate::components::order_detail::OrderDetail;

#[component]
pub fn OrderDetailPage() -> impl IntoView {
    let params = use_params_map();
    let order_id = move || params.with(|p| p.get("order_id").cloned().unwrap_or_default());
    
    view! {
        <div class="order-detail-page">
            <OrderDetail order_id=order_id() />
        </div>
    }
}
