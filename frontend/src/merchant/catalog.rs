// frontend/src/merchant/catalog.rs

use leptos::*;

#[component]
pub fn CatalogPage() -> impl IntoView {
    view! {
        <div class="merchant-catalog-page">
            <div class="catalog-header">
                <h2>"المنتجات"</h2>
                <button class="add-product-btn">"إضافة منتج جديد"</button>
            </div>

            <div class="catalog-filters">
                <input type="text" placeholder="البحث عن منتج..." />
                <select>
                    <option>"جميع الفئات"</option>
                </select>
            </div>

            <CatalogTable />
        </div>
    }
}

#[component]
fn CatalogTable() -> impl IntoView {
    view! {
        <table class="catalog-table">
            <thead>
                <tr>
                    <th>"المنتج"</th>
                    <th>"السعر"</th>
                    <th>"المخزون"</th>
                    <th>"الحالة"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"منتج تجريبي"</td>
                    <td>"99.99 ج.م"</td>
                    <td>"45"</td>
                    <td><span class="badge-active">"منشور"</span></td>
                    <td>
                        <button>"تعديل"</button>
                        <button>"حذف"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
