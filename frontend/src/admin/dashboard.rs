// frontend/src/admin/dashboard.rs

use leptos::*;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <div class="admin-dashboard">
            <h1>"لوحة تحكم الإدارة"</h1>

            <div class="admin-stats">
                <AdminStatCard title="المتاجر النشطة" value="256" />
                <AdminStatCard title="الطلبات اليومية" value="1,234" />
                <AdminStatCard title="المستخدمين" value="12,456" />
                <AdminStatCard title="الإيرادات" value="543,210 ج.م" />
            </div>

            <div class="admin-queues">
                <section class="queue-section">
                    <h3>"طلبات التطبيقات المعلقة"</h3>
                    <p>"23 تطبيق قيد الانتظار"</p>
                </section>

                <section class="queue-section">
                    <h3>"تذاكر الدعم الملحة"</h3>
                    <p>"5 تذاكر تحتاج تصعيد"</p>
                </section>

                <section class="queue-section">
                    <h3>"التقارير والآراء"</h3>
                    <p>"8 تقارير قيد المراجعة"</p>
                </section>
            </div>
        </div>
    }
}

#[component]
fn AdminStatCard(title: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="admin-stat-card">
            <h3>{title}</h3>
            <p class="stat-value">{value}</p>
        </div>
    }
}
