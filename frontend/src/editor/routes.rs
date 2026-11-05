// frontend/src/editor/routes.rs

use leptos::*;
use leptos_router::*;
use serde_json::{json, Value};

use crate::server::puck_api;

#[component]
pub fn EditorPage() -> impl IntoView {
    let params = use_params_map();
    let page_id = move || params.get().get("page_id").cloned();

    let page_data = create_resource(page_id, |id| async move {
        match id {
            Some(page_id) => puck_api::get_page(&page_id).await,
            None => Ok(json!({ "root": [] })),
        }
    });

    let save_page = create_action(|(page_id, content): &(Option<String>, Value)| {
        let page_id = page_id.clone();
        let content = content.clone();
        async move {
            match page_id {
                Some(id) => puck_api::save_page(&id, content).await,
                None => puck_api::create_page(content).await,
            }
        }
    });

    let publish_page = create_action(|page_id: &Option<String>| {
        let page_id = page_id.clone();
        async move {
            match page_id {
                Some(id) => puck_api::publish_page(&id).await,
                None => Err("No page ID".to_string()),
            }
        }
    });

    view! {
        <div class="puck-editor-container">
            <div class="puck-editor-header">
                <h1>"محرر الصفحات"</h1>
                <div class="editor-actions">
                    <button
                        class="btn-save"
                        on:click=move |_| {
                            save_page.dispatch((page_id(), json!({})));
                        }
                    >
                        "حفظ"
                    </button>
                    <button
                        class="btn-publish"
                        on:click=move |_| {
                            publish_page.dispatch(page_id());
                        }
                    >
                        "نشر"
                    </button>
                </div>
            </div>

            <Suspense fallback=|| {
                view! { <div class="loading">"جاري تحميل الصفحة..."</div> }
            }>
                {move || {
                    page_data.and_then(|data| {
                        view! {
                            <div class="puck-editor">
                                <div class="puck-canvas">
                                    <PuckCanvas content=data.clone() />
                                </div>

                                <div class="puck-sidebar">
                                    <ComponentPalette />
                                </div>
                            </div>
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn PuckCanvas(content: Value) -> impl IntoView {
    view! {
        <div class="puck-canvas-content">
            <p>"محرر Puck - جاري الإعداد"</p>
            <pre>{format!("{:#}", content)}</pre>
        </div>
    }
}

#[component]
fn ComponentPalette() -> impl IntoView {
    view! {
        <div class="component-palette">
            <h3>"المكونات"</h3>
            <div class="palette-items">
                <div class="palette-item" draggable="true">
                    "بطل الصفحة"
                </div>
                <div class="palette-item" draggable="true">
                    "قسم"
                </div>
                <div class="palette-item" draggable="true">
                    "شبكة المنتجات"
                </div>
                <div class="palette-item" draggable="true">
                    "صورة"
                </div>
                <div class="palette-item" draggable="true">
                    "زر CTA"
                </div>
            </div>
        </div>
    }
}
