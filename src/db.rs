use sqlx::postgres::{PgPool, PgPoolOptions, PgConnectOptions};
use sqlx::ConnectOptions;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    // Загружаем переменные окружения из .env файла
    dotenv().ok();

    // Получаем DATABASE_URL из переменных окружения
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Парсим URL для подключения
    let connect_options = PgConnectOptions::from_str(&database_url)?;

    // Для проверки существования БД подключаемся к postgres (системной БД)
    let check_db_options = connect_options.clone().database("postgres");

    // Проверяем существование БД
    let pool_temp = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(check_db_options)
        .await?;

    let db_name = connect_options.get_database().unwrap_or("postgres");
    let db_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)"
    )
        .bind(db_name)
        .fetch_one(&pool_temp)
        .await?;

    // Если БД не существует - создаем
    if !db_exists {
        sqlx::query(&format!("CREATE DATABASE \"{}\"", db_name))
            .execute(&pool_temp)
            .await?;
    }

    // Подключаемся к нашей БД
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    // Создаем таблицу tasks, если она не существует
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
        .execute(&pool)
        .await?;

    Ok(pool)
}