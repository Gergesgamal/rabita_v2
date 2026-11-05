// backend/src/authorization/capability.rs

//! Permission catalog loader and capability checking per D-009.

use crate::error::{AppError, AppResult};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

/// Complete permission catalog per D-009.
pub struct CapabilityChecker {
    capabilities: HashMap<String, CapabilityDef>,
}

#[derive(Debug, Clone)]
pub struct CapabilityDef {
    pub name: String,
    pub description: String,
    pub is_delegable: bool,
    pub is_dual_control: bool,
}

impl CapabilityChecker {
    /// Load capability catalog from database.
    pub async fn load(db: &PgPool) -> AppResult<Self> {
        let rows = sqlx::query_as::<_, (String, String, bool, bool)>(
            r#"
            SELECT capability_name, description, is_delegable, is_dual_control
            FROM app.permission_catalog
            ORDER BY capability_name
            "#,
        )
        .fetch_all(db)
        .await?;

        let mut capabilities = HashMap::new();
        for (name, description, is_delegable, is_dual_control) in rows {
            capabilities.insert(
                name.clone(),
                CapabilityDef {
                    name,
                    description,
                    is_delegable,
                    is_dual_control,
                },
            );
        }

        Ok(Self { capabilities })
    }

    /// Check if capability exists in catalog.
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains_key(capability)
    }

    /// Get capability definition.
    pub fn get(&self, capability: &str) -> Option<&CapabilityDef> {
        self.capabilities.get(capability)
    }

    /// List all capabilities.
    pub fn list_all(&self) -> Vec<&CapabilityDef> {
        self.capabilities.values().collect()
    }

    /// Check if capability is delegable per D-009.
    pub fn is_delegable(&self, capability: &str) -> bool {
        self.capabilities
            .get(capability)
            .map(|c| c.is_delegable)
            .unwrap_or(false)
    }

    /// Check if capability requires dual-control per D-026.
    pub fn requires_dual_control(&self, capability: &str) -> bool {
        self.capabilities
            .get(capability)
            .map(|c| c.is_dual_control)
            .unwrap_or(false)
    }
}

/// Catalog of 30 capabilities per D-009.
pub fn default_capabilities() -> Vec<(&'static str, &'static str, bool, bool)> {
    vec![
        ("catalog.read", "Read product catalog", true, false),
        ("catalog.write", "Create/edit products", true, false),
        ("catalog.delete", "Delete products", true, false),
        ("inventory.adjust", "Adjust branch stock", true, false),
        ("orders.read", "View orders", true, false),
        ("orders.fulfill", "Mark orders as shipped/delivered", true, false),
        ("orders.cancel", "Cancel orders", true, false),
        ("orders.refund", "Approve/process refunds", false, true),
        ("customer.read", "View customer data", true, false),
        ("customer.message", "Send transactional messages", true, false),
        ("support.read", "Read support tickets", true, false),
        ("support.respond", "Reply to tickets", true, false),
        ("support.escalate", "Escalate tickets to admin", true, false),
        ("content.edit", "Edit customizable pages", true, false),
        ("content.publish", "Publish pages", true, false),
        ("analytics.view", "View analytics dashboard", true, false),
        ("promotions.manage", "Create/edit discounts", true, false),
        ("jobs.manage", "Post and manage jobs", true, false),
        ("jobs.review", "Review job applications", true, false),
        ("employees.manage", "Hire/fire employees", false, true),
        ("permissions.grant", "Grant/revoke capabilities", false, true),
        ("store.delete", "Delete a store", false, true),
        ("store.settings.billing", "Manage store billing", false, true),
        ("store.transfer", "Transfer store ownership", false, true),
        ("apikeys.manage", "Manage API keys", false, true),
        ("webhooks.manage", "Configure webhooks", false, true),
        ("shipping.configure", "Set shipping rules", true, false),
        ("media.upload", "Upload media", true, false),
        ("components.author", "Create components", true, false),
        ("components.approve", "Approve components", false, false),
    ]
}
