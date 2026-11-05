// frontend/src/components/addresses.rs

use leptos::*;

#[component]
pub fn GovernorateSelect() -> impl IntoView {
    let governorates = vec![
        ("cairo", "القاهرة"),
        ("giza", "الجيزة"),
        ("alexandria", "الإسكندرية"),
        ("aswan", "أسوان"),
    ];

    view! {
        <select class="governorate-select">
            <option value="">"اختر محافظة"</option>
            {governorates.into_iter().map(|(code, name)| {
                view! {
                    <option value=code>{name}</option>
                }
            }).collect::<Vec<_>>()}
        </select>
    }
}

#[component]
pub fn AddressSelector(
    on_select: Box<dyn Fn(String)>,
) -> impl IntoView {
    view! {
        <div class="address-selector">
            <p>"اختر عنوان التوصيل"</p>
            // TODO: Fetch and display saved addresses
        </div>
    }
}
