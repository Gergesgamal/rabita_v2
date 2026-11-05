// frontend/src/merchant/dashboard.rs

use leptos::*;
use leptos_router::*;

#[component]
pub fn MerchantDashboard() -> impl IntoView {
    view! {
        <div class="merchant-dashboard">
            <header class="dashboard-header">
                <h1>"لوحة التحكم"</h1>
                <nav class="dashboard-nav">
                    <A href="/dashboard/orders">"الطلبات"</A>
                    <A href="/dashboard/catalog">"الكتالوج"</A>
                    <A href="/dashboard/inventory">"المخزون"</A>
                    <A href="/dashboard/support">"الدعم"</A>
                    <A href="/dashboard/analytics">"التحليلات"</A>
                    <A href="/dashboard/settings">"الإعدادات"</A>
                </nav>
            </header>

            <main class="dashboard-main">
                <StatCard title="الطلبات اليوم" value="23" />
                <StatCard title="الإيرادات" value="12,450 ج.م" />
                <StatCard title="العملاء" value="156" />
                <StatCard title="المنتجات" value="89" />
            </main>
        </div>
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="stat-card">
            <h3>{title}</h3>
            <p class="stat-value">{value}</p>
        </div>
    }
}
