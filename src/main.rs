use tokio;

mod db;
mod seed;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let db_pool = db::init_db().await;

    let users: i32 = 700_000;
    let pictures: i32 = 30_000;
    let traders: i32 = 20;
    let items: i32 = 50_000;
    let quests: i32 = 1_000;
    let quest_rewards: i32 = 3_000;
    let maps: i32 = 10;
    let comments: i32 = 150_000;
    let map_markers: i32 = 100 * maps;
    let quest_refs: i32 = quests / 2;
    let completed_quests: i32 = users * quests / 4000;
    let quest_maps: i32 = quests * maps / 6;


    if let Err(e) = seed::seed_users(&db_pool, users).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_pictures(&db_pool, pictures).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_traders(&db_pool, traders, pictures).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_items(&db_pool, items, pictures).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_quests(&db_pool, quests, pictures, traders).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_quest_rewards(&db_pool, quest_rewards, quests, items, traders).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_maps(&db_pool, maps, pictures).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_comments(&db_pool, comments, users, quests).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_map_markers(&db_pool, map_markers, maps, pictures).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_quest_refs(&db_pool, quest_refs, quests).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_user_complete_quests(&db_pool, completed_quests, users, quests).await {
        eprintln!("Seed failed: {e}");
    }

    if let Err(e) = seed::seed_quest_map(&db_pool, quest_maps, quests, maps).await {
        eprintln!("Seed failed: {e}");
    }

    Ok(())
}
