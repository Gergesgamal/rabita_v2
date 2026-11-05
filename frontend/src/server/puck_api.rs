// frontend/src/server/puck_api.rs

use leptos::server;
use serde_json::Value;

#[server(GetPage, "/api")]
pub async fn get_page(page_id: String) -> Result<Value, ServerFnError> {
    // Fetch page from backend
    Ok(serde_json::json!({
        "root": []
    }))
}

#[server(SavePage, "/api")]
pub async fn save_page(page_id: String, content: Value) -> Result<String, ServerFnError> {
    // Save page draft
    Ok(page_id)
}

#[server(CreatePage, "/api")]
pub async fn create_page(content: Value) -> Result<String, ServerFnError> {
    // Create new page
    Ok("new-page-id".to_string())
}

#[server(PublishPage, "/api")]
pub async fn publish_page(page_id: String) -> Result<String, ServerFnError> {
    // Publish page
    Ok(page_id)
}

#[server(GetComponentApprovals, "/api")]
pub async fn get_component_approvals() -> Result<Vec<(String, String)>, ServerFnError> {
    // Fetch pending component approvals for admin
    Ok(vec![])
}

#[server(ApproveComponent, "/api")]
pub async fn approve_component(
    component_id: String,
) -> Result<String, ServerFnError> {
    // Approve UI-authored component
    Ok(component_id)
}
