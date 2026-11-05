// frontend/src/merchant/orders.rs

use leptos::*;

#[component]
pub fn OrdersPage() -> impl IntoView {
    view! {
        <div class="merchant-orders-page">
            <h2>"الطلبات"</h2>
            <div class="orders-filters">
                <input type="text" placeholder="البحث عن طلب..." />
                <select>
                    <option>"جميع الحالات"</option>
                    <option>"قيد المراجعة"</option>
                    <option>"مؤكد"</option>
                    <option>"قيد الشحن"</option>
                    <option>"تم التوصيل"</option>
                </select>
            </div>
            <OrdersTable />
        </div>
    }
}

#[component]
fn OrdersTable() -> impl IntoView {
    view! {
        <table class="orders-table">
            <thead>
                <tr>
                    <th>"رقم الطلب"</th>
                    <th>"التاريخ"</th>
                    <th>"المبلغ"</th>
                    <th>"الحالة"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"rabita-ABC123"</td>
                    <td>"2026-10-28"</td>
                    <td>"1,234.56 ج.م"</td>
                    <td><StatusBadge status="shipped" /></td>
                    <td>
                        <button>"عرض"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
fn StatusBadge(status: &'static str) -> impl IntoView {
    let (class, label) = match status {
        "pending" => ("badge-pending", "قيد المراجعة"),
        "confirmed" => ("badge-confirmed", "مؤكد"),
        "shipped" => ("badge-shipped", "قيد الشحن"),
        "delivered" => ("badge-delivered", "تم التوصيل"),
        "cancelled" => ("badge-cancelled", "ملغى"),
        _ => ("badge-pending", "غير معروف"),
    };

    view! {
        <span class=format!("status-badge {}", class)>{label}</span>
    }
}
