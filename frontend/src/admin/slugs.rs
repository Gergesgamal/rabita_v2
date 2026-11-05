// frontend/src/admin/slugs.rs

use leptos::*;

#[component]
pub fn SlugsPage() -> impl IntoView {
    view! {
        <div class="admin-slugs-page">
            <h2>"الرموز المحفوظة"</h2>
            <ReservedSlugsList />
        </div>
    }
}

#[component]
fn ReservedSlugsList() -> impl IntoView {
    view! {
        <table class="slugs-table">
            <thead>
                <tr>
                    <th>"الرمز"</th>
                    <th>"النوع"</th>
                    <th>"السبب"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"rabita"</td>
                    <td>"محفوظ من النظام"</td>
                    <td>"الموقع الرسمي"</td>
                    <td>"عرض"</td>
                </tr>
            </tbody>
        </table>
    }
}
