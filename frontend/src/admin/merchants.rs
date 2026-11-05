// frontend/src/admin/merchants.rs

use leptos::*;

#[component]
pub fn MerchantsPage() -> impl IntoView {
    view! {
        <div class="admin-merchants-page">
            <h2>"إدارة المتاجر"</h2>
            <MerchantsList />
        </div>
    }
}

#[component]
fn MerchantsList() -> impl IntoView {
    view! {
        <table class="merchants-table">
            <thead>
                <tr>
                    <th>"اسم المتجر"</th>
                    <th>"المالك"</th>
                    <th>"الحالة"</th>
                    <th>"تاريخ الإنشاء"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"متجري"</td>
                    <td>"أحمد محمد"</td>
                    <td><span class="badge-active">"نشط"</span></td>
                    <td>"2026-09-01"</td>
                    <td>
                        <button>"عرض"</button>
                        <button class="btn-danger">"إيقاف"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
