// frontend/src/merchant/employees.rs

use leptos::*;

#[component]
pub fn EmployeesPage() -> impl IntoView {
    view! {
        <div class="merchant-employees-page">
            <h2>"الموظفون"</h2>
            <EmployeesTable />
        </div>
    }
}

#[component]
fn EmployeesTable() -> impl IntoView {
    view! {
        <table class="employees-table">
            <thead>
                <tr>
                    <th>"الاسم"</th>
                    <th>"البريد الإلكتروني"</th>
                    <th>"الصلاحيات"</th>
                    <th>"الإجراءات"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"أحمد محمد"</td>
                    <td>"ahmed@example.com"</td>
                    <td>"تنفيذ الطلبات"</td>
                    <td>
                        <button>"تعديل"</button>
                        <button>"حذف"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
