use tokio;

mod db;
mod seed;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let db_pool = db::init_db().await;

    seed::seeds_generate(&db_pool).await;

    Ok(())
}
