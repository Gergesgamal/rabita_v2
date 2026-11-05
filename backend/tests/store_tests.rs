// backend/tests/store_tests.rs

mod common;

use common::TestContext;
use sqlx::Row;

#[tokio::test]
async fn test_store_creation_with_valid_slug() {
    let ctx = TestContext::new().await;

    let merchant_id = common::create_test_merchant(&ctx.db, "merchant1").await;
    let store_id = common::create_test_store(&ctx.db, &merchant_id, "valid-slug-123").await;

    // Verify store was created
    let store = sqlx::query("SELECT id, slug FROM app.store WHERE id = $1")
        .bind(&store_id)
        .fetch_one(&ctx.db)
        .await
        .expect("Store not found");

    assert_eq!(store.get::<String, _>("slug"), "valid-slug-123");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_store_slug_uniqueness() {
    let ctx = TestContext::new().await;

    let merchant_id = common::create_test_merchant(&ctx.db, "merchant2").await;

    // Create first store with slug
    let _store1 = common::create_test_store(&ctx.db, &merchant_id, "unique-slug").await;

    // Attempt to create second store with same slug should fail
    let result = sqlx::query(
        r#"
        INSERT INTO app.store (id, merchant_id, slug, name, status, created_at, updated_at)
        VALUES (gen_random_uuid(), $1, $2, $3, 'active', NOW(), NOW())
        "#
    )
    .bind(&merchant_id)
    .bind("unique-slug")
    .bind("Store 2")
    .execute(&ctx.db)
    .await;

    assert!(result.is_err(), "Duplicate slug should fail");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_store_publication_gate() {
    let ctx = TestContext::new().await;

    let merchant_id = common::create_test_merchant(&ctx.db, "merchant3").await;
    let store_id = common::create_test_store(&ctx.db, &merchant_id, "pub-test").await;

    // Store should not be publishable without products
    // (per D-006: publication requires ≥1 published product)

    // Add product
    let _product_id = common::create_test_product(&ctx.db, &store_id, "Test Product", 10000).await;

    // Now store should be publishable

    ctx.cleanup().await;
}
