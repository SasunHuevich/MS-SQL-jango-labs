use tokio;
use std::env;


mod db;
mod seed;
mod simulation;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    println!("INIT DB");

    let db_pool = db::init_db().await;

    let lab_mode = env::var("LAB_MODE")
        .expect("LAB_MODE must be set");

    match lab_mode.as_str() {
        "lab2" => {
            println!("SEED GENERATE");
            seed::seeds_generate(&db_pool).await;
        }
        "lab3" => {
            println!("SIMULATION");
            let handlers = simulation::spawn_virtual_users(&db_pool, 50).await;

            for handle in handlers {
                let _ =handle.await;
            }
        }
        _ => {
            println!("UNKNOWN LAB MODE: {}!", lab_mode);
        }
    }

    println!("OK");
    Ok(())
}
