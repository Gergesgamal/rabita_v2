use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductItem {
    pub id: String,
    pub name: String,
    pub price: i64,
    pub description: String,
}

#[component]
pub fn StorePage() -> impl IntoView {
    view! { <div class="store-page">"Store Page"</div> }
}
