use leptos::*;
use crate::pages::store::ProductItem;

#[component]
pub fn ProductCard(product: ProductItem) -> impl IntoView {
    view! {
        <div class="product-card">
            <h3>{product.name}</h3>
            <p>{format!("{:.2} EGP", product.price as f64 / 100.0)}</p>
            <p>{product.description}</p>
        </div>
    }
}
