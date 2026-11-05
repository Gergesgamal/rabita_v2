// frontend/src/admin/login.rs

use leptos::*;

#[component]
pub fn AdminLoginPage() -> impl IntoView {
    let (step, set_step) = create_signal::<&str>("password");
    let (error, set_error) = create_signal::<Option<String>>(None);

    view! {
        <div class="admin-login-page">
            <div class="login-card">
                <h1>"دخول الإدارة"</h1>

                {move || match step() {
                    "password" => view! {
                        <form class="login-form">
                            <label>
                                "اسم المستخدم"
                                <input type="text" required />
                            </label>
                            <label>
                                "كلمة المرور"
                                <input type="password" required />
                            </label>
                            <button type="submit">"التالي"</button>
                        </form>
                    }.into_view(),
                    "mfa" => view! {
                        <form class="login-form">
                            <label>
                                "رمز التحقق"
                                <input type="text" placeholder="رمز 6 أرقام" required />
                            </label>
                            <button type="submit">"دخول"</button>
                        </form>
                    }.into_view(),
                    _ => view! { <p>"خطأ"</p> }.into_view(),
                }}

                {error().map(|err| {
                    view! {
                        <div class="error-message">{err}</div>
                    }
                })}
            </div>
        </div>
    }
}
