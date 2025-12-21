use sqlx::MssqlPool;
use dotenv::dotenv;
use std::env;

pub async fn init_db() -> MssqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MssqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to MSSQL")
}