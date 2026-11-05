// frontend/src/pages/account.rs

use leptos::*;
use leptos_router::*;

#[component]
pub fn AccountPages() -> impl IntoView {
    let params = use_params_map();
    let section = move || params.get().get("any").cloned().unwrap_or_default();

    view! {
        <div class="account-pages">
            <AccountNav />
            
            <div class="account-content">
                {move || {
                    match section().as_str() {
                        "profile" => view! { <ProfilePage /> }.into_view(),
                        "orders" => view! { <OrdersPage /> }.into_view(),
                        "addresses" => view! { <AddressesPage /> }.into_view(),
                        "wishlist" => view! { <WishlistPage /> }.into_view(),
                        "support" => view! { <TicketsPage /> }.into_view(),
                        "refunds" => view! { <RefundsPage /> }.into_view(),
                        "settings" => view! { <SettingsPage /> }.into_view(),
                        _ => view! { <OrdersPage /> }.into_view(),
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn AccountNav() -> impl IntoView {
    view! {
        <nav class="account-nav">
            <A href="/account">"الطلبات"</A>
            <A href="/account/profile">"الملف الشخصي"</A>
            <A href="/account/addresses">"العناوين"</A>
            <A href="/account/wishlist">"قائمة الأمنيات"</A>
            <A href="/account/support">"الدعم"</A>
            <A href="/account/refunds">"الاسترجاعات"</A>
            <A href="/account/settings">"الإعدادات"</A>
        </nav>
    }
}

#[component]
fn ProfilePage() -> impl IntoView {
    view! {
        <div class="profile-page">
            <h2>"الملف الشخصي"</h2>
            <p>"تحديث بيانات الملف الشخصي"</p>
        </div>
    }
}

#[component]
fn OrdersPage() -> impl IntoView {
    view! {
        <div class="orders-page">
            <h2>"طلباتي"</h2>
            <p>"قائمة الطلبات"</p>
        </div>
    }
}

#[component]
fn AddressesPage() -> impl IntoView {
    view! {
        <div class="addresses-page">
            <h2>"عناويني"</h2>
            <p>"إدارة عناوين التوصيل"</p>
        </div>
    }
}

#[component]
fn WishlistPage() -> impl IntoView {
    view! {
        <div class="wishlist-page">
            <h2>"قائمة الأمنيات"</h2>
            <p>"المنتجات المحفوظة"</p>
        </div>
    }
}

#[component]
fn TicketsPage() -> impl IntoView {
    view! {
        <div class="tickets-page">
            <h2>"تذاكر الدعم"</h2>
            <p>"طلبات الدعم"</p>
        </div>
    }
}

#[component]
fn RefundsPage() -> impl IntoView {
    view! {
        <div class="refunds-page">
            <h2>"طلبات الاسترجاع"</h2>
            <p>"حالة طلبات الاسترجاع"</p>
        </div>
    }
}

#[component]
fn SettingsPage() -> impl IntoView {
    view! {
        <div class="settings-page">
            <h2>"الإعدادات"</h2>
            <p>"إعدادات الحساب"</p>
        </div>
    }
}
