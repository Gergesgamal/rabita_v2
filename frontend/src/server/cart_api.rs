use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: i32,
    pub price: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub total: i64,
}

pub async fn get_cart(store_slug: &str) -> Result<Cart, String> {
    Ok(Cart {
        items: vec![],
        total: 0,
    })
}

pub async fn update_cart_item(store_slug: &str, product_id: &str, quantity: i32) -> Result<Cart, String> {
    Ok(Cart {
        items: vec![],
        total: 0,
    })
}

pub async fn remove_cart_item(store_slug: &str, product_id: &str) -> Result<Cart, String> {
    Ok(Cart {
        items: vec![],
        total: 0,
    })
}
