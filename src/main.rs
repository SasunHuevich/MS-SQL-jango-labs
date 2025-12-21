use tokio;
use sqlx::Row;

mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let db_pool = db::init_db().await;

    println!("DATABASE_URL={}", std::env::var("DATABASE_URL").unwrap());

    let row = sqlx::query("USE db; SELECT DB_NAME() as current_db")
    .fetch_one(&db_pool)
    .await?;
    let current_db: String = row.get("current_db");
    println!("Connected to database: {}", current_db);

    let row = sqlx::query("USE db; SELECT COUNT(*) as count FROM dbo.picture_type")
        .fetch_one(&db_pool)
        .await?;

    let count: i32 = row.get("count");

    println!("Users count: {}", count);

    Ok(())
}
