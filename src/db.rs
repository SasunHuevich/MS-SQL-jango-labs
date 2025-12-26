use sqlx::MssqlPool;
use dotenv::dotenv;
use std::env;
use sqlx::mssql::MssqlPoolOptions;

pub async fn init_db() -> MssqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = MssqlPoolOptions::new()
    .max_connections(50)
    .connect(&database_url)
    .await
    .expect("Failed to connect to MSSQL");

    pool
}