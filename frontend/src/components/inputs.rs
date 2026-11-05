use leptos::*;

#[component]
pub fn EmailInput() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());
    let (error, set_error) = create_signal::<Option<String>>(None);

    view! {
        <input
            type="email"
            on:change=move |ev| {
                set_value.set(event_target_value(&ev));
                set_error.set(None);
            }
        />
        {move || error.get().map(|err| view! { <p class="error">{err}</p> })}
    }
}

#[component]
pub fn PhoneInput() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());

    view! {
        <input
            type="tel"
            on:change=move |ev| {
                set_value.set(event_target_value(&ev));
            }
        />
    }
}
