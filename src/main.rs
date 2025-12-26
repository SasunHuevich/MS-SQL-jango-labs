use tokio;
use tokio::task::JoinHandle;


mod db;
mod seed;
mod simulation;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    println!("INIT DB");

    let db_pool = db::init_db().await;

    println!("SEED GENERATE");
    //seed::seeds_generate(&db_pool).await;

    println!("SIMULATION");
    let mut handlers = simulation::spawn_virtual_users(&db_pool, 50).await;

    for handle in handlers {
        let _ = handle.await;
    }

    println!("OK");
    Ok(())
}
