// frontend/src/admin/support.rs

use leptos::*;

#[component]
pub fn SupportPage() -> impl IntoView {
    view! {
        <div class="admin-support-page">
            <h2>"تذاكر الدعم"</h2>
            <SupportTicketsTable />
        </div>
    }
}

#[component]
fn SupportTicketsTable() -> impl IntoView {
    view! {
        <table class="support-tickets-table">
            <thead>
                <tr>
                    <th>"التذكرة"</th>
                    <th>"الموضوع"</th>
                    <th>"الأولوية"</th>
                    <th>"الحالة"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"TKT-001"</td>
                    <td>"مشكلة في الشحن"</td>
                    <td>"عالية"</td>
                    <td><span class="badge-urgent">"ملحة"</span></td>
                    <td>
                        <button>"رد"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
