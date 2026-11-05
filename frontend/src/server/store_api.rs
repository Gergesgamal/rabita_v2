use crate::pages::store::{StoreInfo, ProductItem};

pub async fn get_store(slug: String) -> Result<StoreInfo, String> {
    Ok(StoreInfo {
        id: "store-1".to_string(),
        name: "Store".to_string(),
        slug: slug.clone(),
    })
}

pub async fn list_products(store_slug: String) -> Result<Vec<ProductItem>, String> {
    Ok(vec![ProductItem {
        id: "prod-1".to_string(),
        name: "Product".to_string(),
        price: 10000,
        description: "A product".to_string(),
    }])
}
