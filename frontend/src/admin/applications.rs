// frontend/src/admin/applications.rs

use leptos::*;

#[component]
pub fn ApplicationsPage() -> impl IntoView {
    view! {
        <div class="admin-applications-page">
            <h2>"طلبات المتاجر"</h2>
            <ApplicationsList />
        </div>
    }
}

#[component]
fn ApplicationsList() -> impl IntoView {
    view! {
        <table class="applications-table">
            <thead>
                <tr>
                    <th>"اسم المتجر"</th>
                    <th>"البريد الإلكتروني"</th>
                    <th>"التاريخ"</th>
                    <th>"الحالة"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"متجر تجريبي"</td>
                    <td>"admin@store.com"</td>
                    <td>"2026-10-28"</td>
                    <td><span class="badge-pending">"قيد الانتظار"</span></td>
                    <td>
                        <button class="btn-approve">"قبول"</button>
                        <button class="btn-reject">"رفض"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
