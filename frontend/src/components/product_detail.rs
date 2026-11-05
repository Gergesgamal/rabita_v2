use leptos::*;

#[component]
pub fn ProductDetail(#[prop(into)] product_id: String) -> impl IntoView {
    view! { <div class="product-detail">"Product: " {product_id}</div> }
}
