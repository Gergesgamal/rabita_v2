// frontend/src/merchant/store_settings.rs

use leptos::*;

#[component]
pub fn StoreSettingsPage() -> impl IntoView {
    view! {
        <div class="store-settings-page">
            <h2>"إعدادات المتجر"</h2>

            <section class="settings-section">
                <h3>"معلومات المتجر"</h3>
                <form class="settings-form">
                    <label>
                        "اسم المتجر"
                        <input type="text" value="متجري" />
                    </label>
                    <label>
                        "الوصف"
                        <textarea>"وصف المتجر"</textarea>
                    </label>
                    <button type="submit">"حفظ"</button>
                </form>
            </section>

            <section class="settings-section">
                <h3>"الضرائب"</h3>
                <form class="settings-form">
                    <label>
                        "نسبة الضريبة (%)"
                        <input type="number" value="14" min="0" max="100" step="0.01" />
                    </label>
                    <button type="submit">"حفظ"</button>
                </form>
            </section>

            <section class="settings-section">
                <h3>"سياسات الإرجاع"</h3>
                <form class="settings-form">
                    <label>
                        "فترة الإرجاع (أيام)"
                        <input type="number" value="30" min="0" />
                    </label>
                    <button type="submit">"حفظ"</button>
                </form>
            </section>
        </div>
    }
}
