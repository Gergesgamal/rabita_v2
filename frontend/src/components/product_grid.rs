// frontend/src/components/product_grid.rs

use leptos::*;

use super::product_card::ProductCard;
use crate::pages::store::ProductItem;

#[component]
pub fn ProductGrid(products: Vec<ProductItem>) -> impl IntoView {
    view! {
        <div class="product-grid">
            {products.into_iter().map(|product| {
                view! {
                    <ProductCard product />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
