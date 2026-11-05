use leptos::*;
use crate::components::{address_selector::AddressSelector, shipping_selector::ShippingMethodSelector, order_summary::OrderSummaryBox};

#[derive(Clone, Debug)]
pub struct CheckoutState {
    pub items: Vec<String>,
    pub shipping: String,
}

#[component]
pub fn CheckoutPage() -> impl IntoView {
    view! { <div class="checkout-page">"Checkout"</div> }
}
