// frontend/src/merchant/media.rs

use leptos::*;

#[component]
pub fn MediaPage() -> impl IntoView {
    view! {
        <div class="merchant-media-page">
            <h2>"الملفات والصور"</h2>
            <MediaUsage />
            <MediaUpload />
            <MediaGallery />
        </div>
    }
}

#[component]
fn MediaUsage() -> impl IntoView {
    view! {
        <div class="media-usage">
            <h3>"استخدام المساحة"</h3>
            <div class="usage-bar">
                <div class="usage-fill" style="width: 45%;"></div>
            </div>
            <p>"1.35 جيجابايت من 3 جيجابايت"</p>
        </div>
    }
}

#[component]
fn MediaUpload() -> impl IntoView {
    view! {
        <div class="media-upload">
            <h3>"رفع ملفات"</h3>
            <div class="upload-area">
                <p>"اسحب الملفات هنا أو"</p>
                <input type="file" multiple />
            </div>
        </div>
    }
}

#[component]
fn MediaGallery() -> impl IntoView {
    view! {
        <div class="media-gallery">
            <h3>"الملفات"</h3>
            <div class="gallery-grid">
                <div class="gallery-item">
                    <img src="placeholder.jpg" alt="صورة" />
                </div>
            </div>
        </div>
    }
}
