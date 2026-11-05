// frontend/src/components/badges.rs

use leptos::*;

#[component]
pub fn StatusBadge(#[prop(into)] status: String) -> impl IntoView {
    let (class, label) = match status.as_str() {
        "pending" => ("badge-pending", "قيد الانتظار"),
        "confirmed" => ("badge-confirmed", "مؤكد"),
        "shipped" => ("badge-shipped", "قيد الشحن"),
        "delivered" => ("badge-delivered", "تم التوصيل"),
        "cancelled" => ("badge-cancelled", "ملغى"),
        "active" => ("badge-active", "نشط"),
        "inactive" => ("badge-inactive", "غير نشط"),
        _ => ("badge-default", "غير معروف"),
    };

    view! {
        <span class=format!("badge {}", class)>{label}</span>
    }
}

#[component]
pub fn PriorityBadge(#[prop(into)] priority: String) -> impl IntoView {
    let (class, label) = match priority.as_str() {
        "low" => ("priority-low", "منخفضة"),
        "medium" => ("priority-medium", "متوسطة"),
        "high" => ("priority-high", "عالية"),
        "urgent" => ("priority-urgent", "ملحة"),
        _ => ("priority-default", "عادي"),
    };

    view! {
        <span class=format!("priority-badge {}", class)>{label}</span>
    }
}
