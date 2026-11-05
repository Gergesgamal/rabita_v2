// frontend/src/admin/reports.rs

use leptos::*;

#[component]
pub fn ReportsPage() -> impl IntoView {
    view! {
        <div class="admin-reports-page">
            <h2>"التقارير والآراء"</h2>
            <ReportsList />
        </div>
    }
}

#[component]
fn ReportsList() -> impl IntoView {
    view! {
        <table class="reports-table">
            <thead>
                <tr>
                    <th>"التقرير"</th>
                    <th>"النوع"</th>
                    <th>"الحالة"</th>
                    <th>"التاريخ"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"متجر مريب"</td>
                    <td>"إساءة"</td>
                    <td><span class="badge-pending">"قيد المراجعة"</span></td>
                    <td>"2026-10-28"</td>
                    <td>
                        <button>"قبول"</button>
                        <button>"رفض"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
