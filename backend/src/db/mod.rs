use sqlx::PgPool;

pub async fn connect(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(url).await
}

pub async fn create_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    connect(url).await
}
