use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckoutState {
    pub items: Vec<String>,
    pub shipping: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub total: i64,
}

pub async fn validate_checkout(store_slug: &str) -> Result<CheckoutState, String> {
    Ok(CheckoutState {
        items: vec![],
        shipping: "delivery".to_string(),
    })
}

pub async fn create_order(state: CheckoutState) -> Result<Order, String> {
    Ok(Order {
        id: "order-123".to_string(),
        total: 0,
    })
}
