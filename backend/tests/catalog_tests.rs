// backend/tests/catalog_tests.rs

mod common;

use common::TestContext;
use sqlx::Row;

#[tokio::test]
async fn test_product_creation() {
    let ctx = TestContext::new().await;

    let merchant_id = common::create_test_merchant(&ctx.db, "merchant4").await;
    let store_id = common::create_test_store(&ctx.db, &merchant_id, "catalog-test").await;
    let product_id =
        common::create_test_product(&ctx.db, &store_id, "Product 1", 50000).await;

    // Verify product was created
    let product = sqlx::query(
        "SELECT id, base_price_egp FROM app.product WHERE id = $1"
    )
    .bind(&product_id)
    .fetch_one(&ctx.db)
    .await
    .expect("Product not found");

    assert_eq!(product.get::<i64, _>("base_price_egp"), 50000);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_branch_product_stock() {
    let ctx = TestContext::new().await;

    let merchant_id = common::create_test_merchant(&ctx.db, "merchant5").await;
    let store_id = common::create_test_store(&ctx.db, &merchant_id, "stock-test").await;
    let product_id = common::create_test_product(&ctx.db, &store_id, "Stock Product", 20000).await;

    // Create branch
    let branch_id = sqlx::query("INSERT INTO app.branch (id, store_id, name, address) VALUES (gen_random_uuid(), $1, $2, $3) RETURNING id")
        .bind(&store_id)
        .bind("Main Branch")
        .bind("Cairo")
        .fetch_one(&ctx.db)
        .await
        .map(|row| row.get::<String, _>("id"))
        .expect("Failed to create branch");

    // Create branch_product with stock
    sqlx::query(
        "INSERT INTO app.branch_product (branch_id, product_id, stock_qty) VALUES ($1, $2, $3)"
    )
    .bind(&branch_id)
    .bind(&product_id)
    .bind(100)
    .execute(&ctx.db)
    .await
    .expect("Failed to create branch_product");

    ctx.cleanup().await;
}
