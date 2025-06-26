use sqlx::postgres::PgPool;

pub async fn init_db() -> PgPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}