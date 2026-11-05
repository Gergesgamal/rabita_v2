// backend/src/delegation/dual_control.rs

//! Dual-control tier definitions per D-026.

pub const DUAL_CONTROL_CAPABILITIES: &[&str] = &[
    "employees.manage",
    "permissions.grant",
    "store.delete",
    "store.settings.billing",
    "store.transfer",
    "orders.refund",
    "apikeys.manage",
    "webhooks.manage",
];

pub const NON_DELEGABLE_CAPABILITIES: &[&str] = &[
    "permissions.grant",
    "employees.manage",
    "store.delete",
    "store.settings.billing",
    "store.transfer",
    "orders.refund",
    "apikeys.manage",
    "webhooks.manage",
];

pub const SUPER_ADMIN_FLOOR: &[&str] = &[
    "permissions.grant",
    "employees.manage",
    "store.delete",
    "store.transfer",
];

/// Check if capability requires dual-control approval.
pub fn requires_dual_control(capability: &str) -> bool {
    DUAL_CONTROL_CAPABILITIES.contains(&capability)
}

/// Check if capability is non-delegable (can't be granted to others).
pub fn is_non_delegable(capability: &str) -> bool {
    NON_DELEGABLE_CAPABILITIES.contains(&capability)
}

/// Check if capability is super-admin-floor (can't be delegated at all).
pub fn is_super_admin_floor(capability: &str) -> bool {
    SUPER_ADMIN_FLOOR.contains(&capability)
}
