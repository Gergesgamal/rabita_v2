use leptos::*;

#[component]
pub fn TextInput() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());

    view! {
        <input
            type="text"
            on:change=move |ev| {
                set_value.set(event_target_value(&ev));
            }
        />
    }
}

#[component]
pub fn Checkbox() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);

    view! {
        <input
            type="checkbox"
            on:change=move |ev| {
                set_checked.set(event_target_checked(&ev));
            }
        />
    }
}
