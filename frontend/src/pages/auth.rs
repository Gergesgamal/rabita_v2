// frontend/src/pages/auth.rs

use leptos::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="login-page">
            <h1>"تسجيل الدخول"</h1>
            <LoginForm />
        </div>
    }
}

#[component]
pub fn SignupPage() -> impl IntoView {
    view! {
        <div class="signup-page">
            <h1>"إنشاء حساب"</h1>
            <SignupForm />
        </div>
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    view! {
        <form class="login-form">
            <p>"قريبا..."</p>
        </form>
    }
}

#[component]
fn SignupForm() -> impl IntoView {
    view! {
        <form class="signup-form">
            <p>"قريبا..."</p>
        </form>
    }
}
