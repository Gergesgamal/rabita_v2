// frontend/src/components/order_summary.rs

use leptos::*;
use crate::utils::format_egp;

#[component]
pub fn OrderSummaryBox() -> impl IntoView {
    view! {
        <div class="order-summary-box">
            <h3>"ملخص الطلب"</h3>
            <div class="summary-line">
                <span>"الإجمالي قبل الضرائب:"</span>
                <span>"0.00 ج.م"</span>
            </div>
            <div class="summary-line">
                <span>"الضريبة (14%):"</span>
                <span>"0.00 ج.م"</span>
            </div>
            <div class="summary-line">
                <span>"الشحن:"</span>
                <span>"0.00 ج.م"</span>
            </div>
            <div class="summary-line total">
                <span>"الإجمالي:"</span>
                <span>"0.00 ج.م"</span>
            </div>
        </div>
    }
}
