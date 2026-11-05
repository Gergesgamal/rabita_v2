// frontend/src/merchant/refunds.rs

use leptos::*;

#[component]
pub fn RefundsPage() -> impl IntoView {
    view! {
        <div class="merchant-refunds-page">
            <h2>"طلبات الاسترجاع"</h2>
            <RefundsTable />
        </div>
    }
}

#[component]
fn RefundsTable() -> impl IntoView {
    view! {
        <table class="refunds-table">
            <thead>
                <tr>
                    <th>"رقم الطلب"</th>
                    <th>"المبلغ"</th>
                    <th>"الحالة"</th>
                    <th>"التاريخ"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"rabita-ABC123"</td>
                    <td>"500.00 ج.م"</td>
                    <td><span class="badge-pending">"قيد الانتظار"</span></td>
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
