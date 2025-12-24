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


    macro_rules! try_seed {
        ($name:expr, $func:expr $(, $args:expr)*) => {
            if let Err(e) = $func(&db_pool $(, $args)*).await {
                eprintln!("Seed failed for {}: {e}", $name);
            } else {
                println!("Seeding {} done", $name);
            }
        };
    }

    try_seed!("users", seed::seed_users, users);
    try_seed!("pictures", seed::seed_pictures, pictures);
    try_seed!("traders", seed::seed_traders, traders, pictures);
    try_seed!("items", seed::seed_items, items, pictures);
    try_seed!("quests", seed::seed_quests, quests, pictures, traders);
    try_seed!("quest_rewards", seed::seed_quest_rewards, quest_rewards, quests, items, traders);
    try_seed!("maps", seed::seed_maps, maps, pictures);
    try_seed!("comments", seed::seed_comments, comments, users, quests);
    try_seed!("map_markers", seed::seed_map_markers, map_markers, maps, pictures);
    try_seed!("quest_refs", seed::seed_quest_refs, quest_refs, quests);
    try_seed!("user_complete_quests", seed::seed_user_complete_quests, completed_quests, users, quests);
    try_seed!("quest_map", seed::seed_quest_map, quest_maps, quests, maps);

    Ok(())
}
