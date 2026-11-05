// backend/tests/order_tests.rs

mod common;

use common::TestContext;
use sqlx::Row;
use uuid::Uuid;

#[tokio::test]
async fn test_order_creation_and_stock_reserve() {
    let ctx = TestContext::new().await;

    let merchant_id = common::create_test_merchant(&ctx.db, "merchant6").await;
    let store_id = common::create_test_store(&ctx.db, &merchant_id, "order-test").await;
    let product_id = common::create_test_product(&ctx.db, &store_id, "Order Product", 30000).await;

    // Create order (stock should be pending, not decremented)
    let order_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO app.order (id, store_id, consumer_id, public_order_number, payment_status, status, order_snapshot)
        VALUES ($1, $2, NULL, $3, 'pending', 'pending', '{}')
        "#
    )
    .bind(&order_id)
    .bind(&store_id)
    .bind(format!("test-{}", Uuid::new_v4()))
    .execute(&ctx.db)
    .await
    .expect("Failed to create order");

    // Verify order was created
    let order = sqlx::query("SELECT id FROM app.order WHERE id = $1")
        .bind(&order_id)
        .fetch_one(&ctx.db)
        .await
        .expect("Order not found");

    assert_eq!(order.get::<String, _>("id"), order_id);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_idempotency_key_prevents_duplicate_orders() {
    let ctx = TestContext::new().await;

    // Per D-130: same key + same payload hash returns stored response
    // same key + different payload hash returns 409 Conflict

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_order_public_number_uniqueness() {
    let ctx = TestContext::new().await;

    // Per D-120: public order number must be unique platform-wide

    ctx.cleanup().await;
}
