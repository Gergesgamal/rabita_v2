use leptos::*;
use std::rc::Rc;

#[component]
pub fn ShippingMethodSelector(
    on_select: Box<dyn Fn(String)>,
) -> impl IntoView {
    let on_select = Rc::new(on_select);
    let on_select_1 = on_select.clone();
    let on_select_2 = on_select.clone();

    view! {
        <div class="shipping-selector">
            <input
                type="radio"
                name="shipping"
                on:change=move |_ev| {
                    on_select_1("delivery".to_string());
                }
            />
            <input
                type="radio"
                name="shipping"
                on:change=move |_ev| {
                    on_select_2("pickup".to_string());
                }
            />
        </div>
    }
}
