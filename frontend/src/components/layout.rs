// frontend/src/components/layout.rs

use leptos::*;
use leptos_router::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="main-layout" dir="rtl" lang="ar">
            <header class="header">
                <nav class="navbar">
                    <A href="/" class="logo">
                        "رابطة"
                    </A>
                    <div class="nav-links">
                        <A href="/">"الرئيسية"</A>
                        <A href="/account">"الحساب"</A>
                    </div>
                </nav>
            </header>

            <main class="main-content">
                {children()}
            </main>

            <footer class="footer">
                <div class="footer-content">
                    <p>"© 2026 Rabita Commerce Platform"</p>
                    <nav class="footer-links">
                        <A href="/terms">"الشروط"</A>
                        <A href="/privacy">"الخصوصية"</A>
                        <A href="/contact">"اتصل بنا"</A>
                    </nav>
                </div>
            </footer>
        </div>
    }
}
