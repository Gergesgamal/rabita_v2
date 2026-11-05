// backend/tests/common/mod.rs

use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TestContext {
    pub db: PgPool,
    pub base_url: String,
}

impl TestContext {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/rabita_test".to_string());

        let db = PgPool::connect(&database_url)
            .await
            .expect("Failed to create test pool");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .expect("Failed to run migrations");

        TestContext {
            db,
            base_url: "http://localhost:3000".to_string(),
        }
    }

    pub async fn cleanup(&self) {
        // Cleanup test data
    }
}

pub async fn create_test_merchant(db: &PgPool, name: &str) -> String {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
        r#"
        INSERT INTO app.account (id, identifier, account_type, status, created_at, updated_at)
        VALUES ($1, $2, 'merchant', 'active', NOW(), NOW())
        "#
    )
    .bind(&id)
    .bind(format!("mer-{}", name))
    .execute(db)
    .await
    .expect("Failed to create test merchant");

    id
}

pub async fn create_test_store(
    db: &PgPool,
    merchant_id: &str,
    slug: &str,
) -> String {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO app.store (id, merchant_id, slug, name, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'active', NOW(), NOW())
        "#
    )
    .bind(&id)
    .bind(merchant_id)
    .bind(slug)
    .bind(slug)
    .execute(db)
    .await
    .expect("Failed to create test store");

    id
}

pub async fn create_test_product(
    db: &PgPool,
    store_id: &str,
    name: &str,
    price_egp: i64,
) -> String {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO app.product (id, store_id, name, base_price_egp, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'published', NOW(), NOW())
        "#
    )
    .bind(&id)
    .bind(store_id)
    .bind(name)
    .bind(price_egp)
    .execute(db)
    .await
    .expect("Failed to create test product");

    id
}

pub async fn create_test_consumer(
    db: &PgPool,
    store_id: &str,
    email: &str,
) -> String {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO app.consumer (id, store_id, normalized_email, created_at)
        VALUES ($1, $2, $3, NOW())
        "#
    )
    .bind(&id)
    .bind(store_id)
    .bind(email.to_lowercase())
    .execute(db)
    .await
    .expect("Failed to create test consumer");

    id
}
