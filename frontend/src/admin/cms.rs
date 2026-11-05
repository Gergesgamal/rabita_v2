// frontend/src/admin/cms.rs

use leptos::*;

#[component]
pub fn CMSPage() -> impl IntoView {
    view! {
        <div class="admin-cms-page">
            <h2>"إدارة الصفحات"</h2>
            <CMSPagesList />
        </div>
    }
}

#[component]
fn CMSPagesList() -> impl IntoView {
    view! {
        <table class="cms-pages-table">
            <thead>
                <tr>
                    <th>"الصفحة"</th>
                    <th>"الحالة"</th>
                    <th>"آخر تحديث"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"الشروط والأحكام"</td>
                    <td><span class="badge-published">"منشورة"</span></td>
                    <td>"2026-10-01"</td>
                    <td>
                        <button>"تعديل"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
