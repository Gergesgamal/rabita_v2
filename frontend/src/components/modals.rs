use leptos::*;

#[component]
pub fn Modal(
    #[prop(into)] is_open: ReadSignal<bool>,
    children: Children,
) -> impl IntoView {
    let children = children();
    view! {
        {move || {
            if is_open.get() {
                view! {
                    <div class="modal">
                        {children.clone()}
                    </div>
                }
                .into_view()
            } else {
                view! { <></> }.into_view()
            }
        }}
    }
}
