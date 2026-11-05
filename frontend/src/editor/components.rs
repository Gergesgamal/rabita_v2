// frontend/src/editor/components.rs

use leptos::*;
use serde_json::json;

/// Built-in Puck component: Hero
/// Per D-008: permitted on homepage and landing pages only
#[component]
pub fn HeroComponent(
    title: String,
    description: Option<String>,
    #[prop(default = "#000000")] accent_color: String,
) -> impl IntoView {
    view! {
        <section class="puck-hero" style=format!("--accent-color: {}", accent_color)>
            <div class="hero-content">
                <h1>{title}</h1>
                {description.map(|desc| {
                    view! {
                        <p>{desc}</p>
                    }
                })}
            </div>
        </section>
    }
}

/// Built-in Puck component: Section
#[component]
pub fn SectionComponent(
    title: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="puck-section">
            {title.map(|t| {
                view! {
                    <h2>{t}</h2>
                }
            })}
            <div class="section-content">
                {children()}
            </div>
        </section>
    }
}

/// Built-in Puck component: ProductGrid
#[component]
pub fn ProductGridComponent(
    title: Option<String>,
    category_id: Option<String>,
) -> impl IntoView {
    view! {
        <section class="puck-product-grid">
            {title.map(|t| {
                view! {
                    <h2>{t}</h2>
                }
            })}
            <div class="grid">
                <p>"جاري تحميل المنتجات..."</p>
            </div>
        </section>
    }
}

/// Built-in Puck component: Rich Text
#[component]
pub fn RichTextComponent(
    #[prop(into)] content: String,
) -> impl IntoView {
    view! {
        <div class="puck-rich-text" inner_html=content />
    }
}

/// Built-in Puck component: Image
#[component]
pub fn ImageComponent(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(default = "100%")] width: String,
) -> impl IntoView {
    view! {
        <div class="puck-image">
            <img src=src alt=alt style=format!("width: {}", width) />
        </div>
    }
}

/// Built-in Puck component: Spacer
#[component]
pub fn SpacerComponent(
    #[prop(default = 32)] height_px: i32,
) -> impl IntoView {
    view! {
        <div class="puck-spacer" style=format!("height: {}px", height_px) />
    }
}

/// Built-in Puck component: CTA Button/Banner
#[component]
pub fn CTAComponent(
    #[prop(into)] text: String,
    #[prop(into)] link: String,
    #[prop(default = "button")] style_type: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("puck-cta puck-cta-{}", style_type)>
            <a href=link class="cta-link">{text}</a>
        </div>
    }
}

/// Built-in Puck component: Newsletter Signup
#[component]
pub fn NewsletterComponent() -> impl IntoView {
    view! {
        <div class="puck-newsletter">
            <h3>"اشترك في نشرتنا البريدية"</h3>
            <form>
                <input type="email" placeholder="بريدك الإلكتروني" required />
                <button type="submit">"اشترك"</button>
            </form>
        </div>
    }
}
