use tokio;

mod db;
mod seed;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let db_pool = db::init_db().await;

    if let Err(e) = seed::seed_users(&db_pool, 700_000).await {
        eprintln!("Seed failed: {e}");
    }

    Ok(())
}
