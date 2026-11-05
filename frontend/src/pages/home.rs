// frontend/src/pages/home.rs

use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-page">
            <section class="hero">
                <div class="hero-content">
                    <h1>"اختر متجرك المفضل"</h1>
                    <p>"منصة رابطة للتجارة الإلكترونية"</p>
                    <A href="/stores" class="cta-button">
                        "تصفح المتاجر"
                    </A>
                </div>
            </section>

            <section class="features">
                <div class="feature-card">
                    <h3>"متاجر محلية"</h3>
                    <p>"ادعم المتاجر المحلية والعلامات التجارية المصرية"</p>
                </div>
                <div class="feature-card">
                    <h3>"شحن سهل"</h3>
                    <p>"توصيل في الوقت المتفق عليه أو استلام من الفرع"</p>
                </div>
                <div class="feature-card">
                    <h3>"دفع آمن"</h3>
                    <p>"دفع عند الاستقبال أو طرق دفع آمنة"</p>
                </div>
            </section>
        </div>
    }
}
