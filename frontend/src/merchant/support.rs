// frontend/src/merchant/support.rs

use leptos::*;

#[component]
pub fn SupportPage() -> impl IntoView {
    view! {
        <div class="merchant-support-page">
            <h2>"تذاكر الدعم"</h2>
            <SupportTicketsTable />
        </div>
    }
}

#[component]
fn SupportTicketsTable() -> impl IntoView {
    view! {
        <table class="support-table">
            <thead>
                <tr>
                    <th>"رقم التذكرة"</th>
                    <th>"الموضوع"</th>
                    <th>"الحالة"</th>
                    <th>"تاريخ الإنشاء"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"TKT-001"</td>
                    <td>"سؤال عن الشحن"</td>
                    <td><span class="badge-pending">"قيد الانتظار"</span></td>
                    <td>"2026-10-28"</td>
                    <td>
                        <button>"رد"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
