// frontend/src/merchant/analytics.rs

use leptos::*;

#[component]
pub fn AnalyticsPage() -> impl IntoView {
    view! {
        <div class="merchant-analytics-page">
            <h2>"التحليلات"</h2>
            
            <section class="analytics-section">
                <h3>"المبيعات"</h3>
                <div class="chart-placeholder">"رسم بياني للمبيعات"</div>
            </section>

            <section class="analytics-section">
                <h3>"الزيارات"</h3>
                <div class="chart-placeholder">"رسم بياني للزيارات"</div>
            </section>

            <section class="analytics-section">
                <h3>"أعلى المنتجات"</h3>
                <TopProductsTable />
            </section>
        </div>
    }
}

#[component]
fn TopProductsTable() -> impl IntoView {
    view! {
        <table class="analytics-table">
            <thead>
                <tr>
                    <th>"المنتج"</th>
                    <th>"عدد المبيعات"</th>
                    <th>"الإيرادات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"منتج 1"</td>
                    <td>"123"</td>
                    <td>"12,300 ج.م"</td>
                </tr>
            </tbody>
        </table>
    }
}
