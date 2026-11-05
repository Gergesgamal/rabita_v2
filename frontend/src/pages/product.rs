use leptos::*;
use leptos_router::*;
use crate::components::product_detail::ProductDetail;

#[component]
pub fn ProductDetailPage() -> impl IntoView {
    let params = use_params_map();
    let product_id = move || params.with(|p| p.get("product_id").cloned().unwrap_or_default());
    
    view! {
        <div class="product-detail-page">
            <ProductDetail product_id=product_id() />
        </div>
    }
}
