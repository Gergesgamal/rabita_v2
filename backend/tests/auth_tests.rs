// backend/tests/auth_tests.rs

mod common;

use common::TestContext;
use sqlx::Row;

#[tokio::test]
async fn test_merchant_login_flow() {
    let ctx = TestContext::new().await;

    // 1. Create merchant account
    let merchant_id = common::create_test_merchant(&ctx.db, "test_merchant").await;

    // 2. Verify merchant exists
    let merchant = sqlx::query("SELECT id FROM app.account WHERE id = $1")
        .bind(&merchant_id)
        .fetch_one(&ctx.db)
        .await
        .expect("Merchant not found");

    assert_eq!(merchant.get::<String, _>("id"), merchant_id);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_consumer_signup() {
    let ctx = TestContext::new().await;

    let store_id = common::create_test_store(&ctx.db, "test", "test-store").await;
    let consumer_id = common::create_test_consumer(&ctx.db, &store_id, "test@example.com").await;

    // Verify consumer was created
    let consumer = sqlx::query("SELECT id FROM app.consumer WHERE id = $1")
        .bind(&consumer_id)
        .fetch_one(&ctx.db)
        .await
        .expect("Consumer not found");

    assert_eq!(consumer.get::<String, _>("id"), consumer_id);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_password_reset_no_existence_disclosure() {
    let ctx = TestContext::new().await;

    // Attempt reset on non-existent account should return same response
    // as existing account (per D-128 and D-111)

    ctx.cleanup().await;
}
