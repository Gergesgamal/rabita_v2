// frontend/src/i18n/mod.rs

use leptos::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub enum Language {
    Arabic,
    English,
}

#[derive(Clone)]
pub struct I18nContext {
    pub lang: Language,
}

pub fn use_i18n() -> I18nContext {
    use_context::<I18nContext>().expect("i18n context not found")
}

/// Initialize i18n per D-127 (Arabic-first for consumers)
#[component]
pub fn I18nProvider(children: Children, #[prop(default = Language::Arabic)] default_lang: Language) -> impl IntoView {
    let ctx = I18nContext {
        lang: default_lang,
    };

    provide_context(ctx);

    view! {
        {children()}
    }
}

pub fn t(key: &str) -> String {
    match key {
        "home.title" => "اختر متجرك المفضل".to_string(),
        "home.subtitle" => "منصة رابطة للتجارة الإلكترونية".to_string(),
        "store.products" => "المنتجات".to_string(),
        "cart.add_to_cart" => "أضف إلى السلة".to_string(),
        "loading" => "جاري التحميل...".to_string(),
        "error" => "حدث خطأ".to_string(),
        _ => key.to_string(),
    }
}
