// frontend/src/admin/admin_management.rs

use leptos::*;

#[component]
pub fn AdminManagementPage() -> impl IntoView {
    view! {
        <div class="admin-management-page">
            <h2>"إدارة المسؤولين"</h2>
            <AdminsList />
        </div>
    }
}

#[component]
fn AdminsList() -> impl IntoView {
    view! {
        <table class="admins-table">
            <thead>
                <tr>
                    <th>"الاسم"</th>
                    <th>"البريد الإلكتروني"</th>
                    <th>"الصلاحيات"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"المسؤول الرئيسي"</td>
                    <td>"admin@rabita.shop"</td>
                    <td>"مسؤول"</td>
                    <td>"عرض"</td>
                </tr>
            </tbody>
        </table>
    }
}
