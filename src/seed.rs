use sqlx::{Pool, Mssql};
use fake::faker::internet::en::{SafeEmail, Username, Password};
use fake::{Fake, faker::{lorem::en::Sentences, company::en::CompanyName}};
use rand::{Rng, prelude::IndexedRandom};
use chrono::{Utc, Duration};
use std::collections::HashSet;

pub async fn seed_users(
    pool: &Pool<Mssql>,
    n: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    let role_ids: Vec<i32> = sqlx::query_scalar("USE db; SELECT id FROM dbo.roles")
        .fetch_all(pool)
        .await?;

    if role_ids.is_empty() {
        panic!("Таблица roles пуста");
    }

    let min_days = 20 * 365;
    let max_days = 30 * 365;

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let username = format!(
                "{}_{}",
                Username().fake::<String>(),
                rng.random::<u32>()
            );

            let email: String = SafeEmail().fake();

            let password: String = Password(8..16).fake();

            let role_id = *role_ids.choose(&mut rng).unwrap();

            let days_ago = rng.random_range(min_days..=max_days);
            let registered_at = Utc::now() - Duration::days(days_ago);
            let registered_at_str = registered_at.format("%Y-%m-%d %H:%M:%S").to_string();

            sqlx::query(
                r#"
                USE db; INSERT INTO dbo.users
                (username, password, email, role_id, registred_at)
                VALUES (@p1, @p2, @p3, @p4, @p5)
                "#
            )
            .bind(username)
            .bind(password)
            .bind(email)
            .bind(role_id)
            .bind(registered_at_str)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} users",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_pictures(
    pool: &Pool<Mssql>,
    n: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    let type_ids: Vec<i32> = sqlx::query_scalar(
        r#"
        USE db;
        SELECT id FROM dbo.picture_type
        "#
    )
    .fetch_all(pool)    
    .await?;

    if type_ids.is_empty() {
        panic!("Таблица picture_type пуста");
    }

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let type_id = *type_ids.choose(&mut rng).unwrap();

            let url: String = format!(
                "/opt/assistent/img/{}/{}",
                type_id,
                rng.random::<u32>()
            );

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.picture (url, type_id)
                VALUES (@p1, @p2)
                "#
            )
            .bind(url)
            .bind(type_id)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} pictures",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_traders(
    pool: &Pool<Mssql>,
    n: i32,
    max_picture_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let name = Username().fake::<String>();

            let description: String = Sentences(5..10)
                .fake::<Vec<String>>()
                .join(" ")
                .chars()
                .take(500)
                .collect();

            let picture_id = rng.random_range(1..=max_picture_id);

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.trader (name, description, picture_id)
                VALUES (@p1, @p2, @p3)
                "#
            )
            .bind(name)
            .bind(description)
            .bind(picture_id)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} traders",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_items(
    pool: &Pool<Mssql>,
    n: i32,
    max_picture_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let name = Username().fake::<String>();

            let description: String = Sentences(5..10)
                .fake::<Vec<String>>()
                .join(" ")
                .chars()
                .take(500)
                .collect();

            let picture_id = rng.random_range(1..=max_picture_id);

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.item (name, description, picture_id)
                VALUES (@p1, @p2, @p3)
                "#
            )
            .bind(name)
            .bind(description)
            .bind(picture_id)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} items",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_quests(
    pool: &Pool<Mssql>,
    n: i32,
    max_picture_id: i32,
    max_trader_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let name: String = CompanyName().fake();

            let description: String = Sentences(8..15)
                .fake::<Vec<String>>()
                .join(" ");

            let picture_id = rng.random_range(1..=max_picture_id);
            let trader_id = rng.random_range(1..=max_trader_id);
            let required_level = rng.random_range(1..=60);

            let min_days = 20 * 365;
            let max_days = 30 * 365;	

            let days_ago = rng.random_range(min_days..=max_days);
            let changed_at = Utc::now() - Duration::days(days_ago);
            let changed_at_str = changed_at.format("%Y-%m-%d %H:%M:%S").to_string();

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.quest
                    (name, description, picture_id, trader_id, required_level, changed_at)
                VALUES
                    (@p1, @p2, @p3, @p4, @p5, @p6)
                "#
            )
            .bind(name)
            .bind(description)
            .bind(picture_id)
            .bind(trader_id)
            .bind(required_level)
            .bind(changed_at_str)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} quests",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_quest_rewards(
    pool: &Pool<Mssql>,
    n: i32,
    max_quest_id: i32,
    max_item_id: i32,
    max_trader_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    let reward_types = ["item", "reputation", "money"];

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let quest_id = rng.random_range(1..=max_quest_id);
            let reward_type = reward_types.choose(&mut rng).unwrap();
            let item_id = rng.random_range(1..=max_item_id);
            let trader_id = rng.random_range(1..=max_trader_id);
            let amount = rng.random_range(1..=500);
            let reputation_amount = rng.random_range(1..=1000);

            
            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.quest_reward
                    (quest_id, type, item_id, trader_id, amount, reputation_amount)
                VALUES
                    (@p1, @p2, @p3, @p4, @p5, @p6)
                "#
            )
            .bind(quest_id)
            .bind(reward_type)
            .bind(item_id)
            .bind(trader_id)
            .bind(amount)
            .bind(reputation_amount)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} quest rewards",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_maps(
    pool: &Pool<Mssql>,
    n: i32,
    max_picture_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let name: String = CompanyName().fake();

            let description: String = Sentences(2..5)
                .fake::<Vec<String>>()
                .join(" ")
                .chars()
                .take(500)
                .collect();
            let picture_id = rng.random_range(1..=max_picture_id);

            let difficulty = rng.random_range(1..=10);

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.map (name, description, picture_id, difficulty)
                VALUES (@p1, @p2, @p3, @p4)
                "#
            )
            .bind(name)
            .bind(description)
            .bind(picture_id)
            .bind(difficulty)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} maps",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_comments(
    pool: &Pool<Mssql>,
    n: i32,
    max_author_id: i32,
    max_quest_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let text: String = Sentences(2..5)
                .fake::<Vec<String>>()
                .join(" ");

            let author_id = rng.random_range(1..=max_author_id);

            let quest_id = rng.random_range(1..=max_quest_id);

            let rating: Option<i32> = if rng.random_bool(0.8) {
                Some(rng.random_range(1..=5))
            } else {
                None
            };

            let min_days = 20 * 365;
            let max_days = 30 * 365;	

            let days_ago = rng.random_range(min_days..=max_days);
            let changed_at = Utc::now() - Duration::days(days_ago);
            let changed_at_str = changed_at.format("%Y-%m-%d %H:%M:%S").to_string();

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.comment
                    (text, author_id, rating, quest_id, changed_at)
                VALUES (@p1, @p2, @p3, @p4, @p5)
                "#
            )
            .bind(text)
            .bind(author_id)
            .bind(rating)
            .bind(quest_id)
            .bind(changed_at_str)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} comments",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_map_markers(
    pool: &Pool<Mssql>,
    n: i32,
    max_map_id: i32,
    max_picture_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    let marker_types = ["npc", "quest", "resource", "exit"];

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let map_id = rng.random_range(1..=max_map_id);

            let type_name = marker_types.choose(&mut rng).unwrap().to_string();

            let description: String = Sentences(2..5)
                .fake::<Vec<String>>()
                .join(" ")
                .chars()
                .take(500)
                .collect();

            let access_rule: String = format!("level_{}", rng.random_range(1..=50));

            let picture_id: i32 = rng.random_range(1..=max_picture_id);

            let x: f64 = rng.random_range(0.0..100.0);
            let y: f64 = rng.random_range(0.0..100.0);

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.map_marker
                    (map_id, type, description, access_rule, picture_id, x, y)
                VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7)
                "#
            )
            .bind(map_id)
            .bind(type_name)
            .bind(description)
            .bind(access_rule)
            .bind(picture_id)
            .bind(x)
            .bind(y)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} map markers",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_quest_refs(
    pool: &Pool<Mssql>,
    n: i32,
    max_quest_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    use std::collections::HashSet;
    let mut existing_pairs = HashSet::new();

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let (mut quest_id, mut required_quest_id);
            loop {
                quest_id = rng.random_range(1..=max_quest_id);
                required_quest_id = rng.random_range(1..=max_quest_id);
                if quest_id != required_quest_id && !existing_pairs.contains(&(quest_id, required_quest_id)) {
                    existing_pairs.insert((quest_id, required_quest_id));
                    break;
                }
            }

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.quest_ref
                    (quest_id, required_quest_id)
                VALUES (@p1, @p2)
                "#
            )
            .bind(quest_id)
            .bind(required_quest_id)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} quest references",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_user_complete_quests(
    pool: &Pool<Mssql>,
    n: i32,
    max_user_id: i32,
    max_quest_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    let mut existing_pairs = HashSet::new();

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let (mut user_id, mut quest_id);
            loop {
                user_id = rng.random_range(1..=max_user_id);
                quest_id = rng.random_range(1..=max_quest_id);
                if !existing_pairs.contains(&(user_id, quest_id)) {
                    existing_pairs.insert((user_id, quest_id));
                    break;
                }
            }

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.user_complete_quest
                    (user_id, quest_id)
                VALUES (@p1, @p2)
                "#
            )
            .bind(user_id)
            .bind(quest_id)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} user_complete_quest",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}


pub async fn seed_quest_map(
    pool: &Pool<Mssql>,
    n: i32,
    max_quest_id: i32,
    max_map_id: i32,
) -> Result<(), sqlx::Error> {
    let mut rng = rand::rng();

    const BATCH_SIZE: usize = 1_000;
    let n_usize = n as usize;

    let mut existing_pairs = HashSet::new();

    for offset in (0..n_usize).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n_usize - offset) {
            let (mut quest_id, mut map_id);
            loop {
                quest_id = rng.random_range(1..=max_quest_id);
                map_id = rng.random_range(1..=max_map_id);
                if !existing_pairs.contains(&(quest_id, map_id)) {
                    existing_pairs.insert((quest_id, map_id));
                    break;
                }
            }

            sqlx::query(
                r#"
                USE db;
                INSERT INTO dbo.quest_map
                    (quest_id, map_id)
                VALUES (@p1, @p2)
                "#
            )
            .bind(quest_id)
            .bind(map_id)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "Inserted {} / {} quest_map",
            (offset + BATCH_SIZE).min(n_usize),
            n
        );
    }

    Ok(())
}

pub async fn seeds_generate(pool: &Pool<Mssql>) {
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
            if let Err(e) = $func(&pool $(, $args)*).await {
                eprintln!("Seed failed for {}: {e}", $name);
            } else {
                println!("Seeding {} done", $name);
            }
        };
    }

    try_seed!("users", seed_users, users);
    try_seed!("pictures", seed_pictures, pictures);
    try_seed!("traders", seed_traders, traders, pictures);
    try_seed!("items", seed_items, items, pictures);
    try_seed!("quests", seed_quests, quests, pictures, traders);
    try_seed!("quest_rewards", seed_quest_rewards, quest_rewards, quests, items, traders);
    try_seed!("maps", seed_maps, maps, pictures);
    try_seed!("comments", seed_comments, comments, users, quests);
    try_seed!("map_markers", seed_map_markers, map_markers, maps, pictures);
    try_seed!("quest_refs", seed_quest_refs, quest_refs, quests);
    try_seed!("user_complete_quests", seed_user_complete_quests, completed_quests, users, quests);
    try_seed!("quest_map", seed_quest_map, quest_maps, quests, maps);
}