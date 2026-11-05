// frontend/src/app.rs

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{home::Home, store::StorePage};
use crate::components::layout::MainLayout;
use crate::i18n::use_i18n;

#[component]
pub fn App() -> impl IntoView {
    let i18n = use_i18n();

    // Inject styles and metadata
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rabita_frontend.css" />
        <Meta name="charset" content="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="description" content="Rabita Commerce Platform" />
        <Meta name="theme-color" content="#000000" />
        <Title text="Rabita" />

        <MainLayout>
            <Router>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/:store_slug" view=StorePage />
                    <Route path="/:store_slug/product/:product_id" view=ProductPage />
                    <Route path="/:store_slug/cart" view=CartPage />
                    <Route path="/:store_slug/checkout" view=CheckoutPage />
                    <Route path="/account/*any" view=AccountPages />
                    <Route path="*" view=NotFound />
                </Routes>
            </Router>
        </MainLayout>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 Not Found"</h1>
        </div>
    }
}

#[component]
fn ProductPage() -> impl IntoView {
    // TODO: Implement product detail page
    view! {
        <div>"Product Page"</div>
    }
}

#[component]
fn CartPage() -> impl IntoView {
    // TODO: Implement cart page
    view! {
        <div>"Cart"</div>
    }
}

#[component]
fn CheckoutPage() -> impl IntoView {
    // TODO: Implement checkout page
    view! {
        <div>"Checkout"</div>
    }
}

#[component]
fn AccountPages() -> impl IntoView {
    // TODO: Implement account pages
    view! {
        <div>"Account"</div>
    }
}
