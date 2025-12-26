use sqlx::{Mssql, Pool};
use tokio;
use fake::faker::internet::en::{SafeEmail, Username, Password};
use fake::{Fake, faker::{lorem::en::Sentences, company::en::CompanyName}};
use tokio::task::JoinHandle;



pub async fn spawn_virtual_users(pool: &Pool<Mssql>, count: usize) -> Vec<JoinHandle<()>>{
    let mut handlers = Vec::with_capacity(count);
    for _ in 0..count {
        let pool = pool.clone();
        let handle = tokio::spawn(async move {
            loop {
                let choice = rand::random::<u32>() % 100;

                match choice {
                    0..=19 => {
                        // println!("0..=19");

                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM users")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let user_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("SELECT * FROM users WHERE id=@p1")
                            .bind(user_id)
                            .fetch_one(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Пользователь с id={} выбран", user_id),
                            Err(e) => eprintln!("[ERROR] Не удалось выбрать пользователя с id={}: {}", user_id, e),
                        }
                    },
                    20..=29 => {
                        // println!("20..=29");
                        
                        let username = Username().fake::<String>();
                        let email: String = SafeEmail().fake();
                        let password: String = Password(8..16).fake();

                        let res = sqlx::query("INSERT INTO users (username, password, email) VALUES (@p1,@p2,@p3)")
                            .bind(&username)
                            .bind(&password)
                            .bind(&email)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Пользователь {} добавлен", username),
                            Err(e) => eprintln!("[ERROR] Не удалось добавить пользователя {}: {}", username, e),
                        }
                    },
                    30..=34 => {
                        // println!("30..=34");
                        
                        let password: String = Password(8..16).fake();
                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM users")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let user_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("UPDATE users SET password=@p1 WHERE id=@p2")
                            .bind(&password)
                            .bind(user_id)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Пароль пользователя с id={} обновлён", user_id),
                            Err(e) => eprintln!("[ERROR] Не удалось обновить пароль пользователя с id={}: {}", user_id, e),
                        }
                    },
                    35..=36 => {
                        // println!("35..=36");
                        
                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM users")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let user_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("DELETE FROM users WHERE id=@p1")
                            .bind(user_id)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Пользователь с id={} удалён", user_id),
                            Err(e) => eprintln!("[ERROR] Не удалось удалить пользователя с id={}: {}", user_id, e),
                        }
                    },

                    37..=46 => {
                        // println!("37..=46");
                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM quest")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let quest_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("SELECT id, name, required_level, LEFT(description, 200) AS description FROM quest WHERE id=@p1")
                            .bind(quest_id)
                            .fetch_one(&pool)
                            .await;

                        match res {
                            Ok(_) => println!("[OK] Квест с id={} выбран", quest_id),
                            Err(e) => eprintln!("[ERROR] Не удалось выбрать квест с id={}: {}", quest_id, e),
                        }
                    },
                    47..=51 => {
                        // println!("47..=51");

                        let name: String = CompanyName().fake();
                        let description: String = Sentences(8..15).fake::<Vec<String>>().join(" ");
                        let required_level = (rand::random::<u32>() % 60) as i32 + 1;

                        let res = sqlx::query("INSERT INTO quest (name, description, required_level) VALUES (@p1,@p2,@p3)")
                            .bind(&name)
                            .bind(&description)
                            .bind(required_level)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Квест '{}' добавлен", name),
                            Err(e) => eprintln!("[ERROR] Не удалось добавить квест '{}': {}", name, e),
                        }
                    },
                    52..=56 => {
                        // println!("52..=56");

                        let description: String = Sentences(8..15).fake::<Vec<String>>().join(" ");
                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM quest")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let quest_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("UPDATE quest SET description=@p1 WHERE id=@p2")
                            .bind(&description)
                            .bind(quest_id)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Квест с id={} обновлён", quest_id),
                            Err(e) => eprintln!("[ERROR] Не удалось обновить квест с id={}: {}", quest_id, e),
                        }
                    },

                    57..=61 => {
                        // println!("57..=61");

                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM item")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let item_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("SELECT * FROM item WHERE id=@p1")
                            .bind(item_id)
                            .fetch_one(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Предмет с id={} выбран", item_id),
                            Err(e) => eprintln!("[ERROR] Не удалось выбрать предмет с id={}: {}", item_id, e),
                        }
                    },

                    62..=66 => {
                        // println!("62..=66");

                        let name = Username().fake::<String>();
                        let description: String = Sentences(5..10).fake::<Vec<String>>().join(" ").chars().take(500).collect();

                        let res = sqlx::query("INSERT INTO item (name, description) VALUES (@p1,@p2)")
                            .bind(&name)
                            .bind(&description)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Предмет '{}' добавлен", name),
                            Err(e) => eprintln!("[ERROR] Не удалось добавить предмет '{}': {}", name, e),
                        }
                    },

                    67..=71 => {
                        // println!("67..=71");

                        let text: String = Sentences(2..5).fake::<Vec<String>>().join(" ");
                        let max_author_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM users").fetch_one(&pool).await.unwrap_or(1);
                        let max_quest_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM quest").fetch_one(&pool).await.unwrap_or(1);
                        let author_id = (rand::random::<u32>() % max_author_id as u32) as i32 + 1;
                        let quest_id = (rand::random::<u32>() % max_quest_id as u32) as i32 + 1;

                        let res = sqlx::query("INSERT INTO comment (text, author_id, quest_id) VALUES (@p1,@p2,@p3)")
                            .bind(&text)
                            .bind(author_id)
                            .bind(quest_id)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Комментарий '{}' добавлен", text),
                            Err(e) => eprintln!("[ERROR] Не удалось добавить комментарий '{}': {}", text, e),
                        }
                    },
                    72..=76 => {
                        // println!("72..=76");
                        
                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM comment")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let comment_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("SELECT id, author_id, quest_id, LEFT(text, 200) AS text FROM comment WHERE id=@p1")
                            .bind(comment_id)
                            .fetch_one(&pool)
                            .await;

                        match res {
                            Ok(_) => println!("[OK] Комментарий с id={} выбран", comment_id),
                            Err(e) => eprintln!("[ERROR] Не удалось выбрать комментарий с id={}: {}", comment_id, e),
                        }
                    },
                    77..=81 => {
                        // println!("77..=81");
                        
                        let text: String = Sentences(2..5).fake::<Vec<String>>().join(" ");
                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM comment")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let comment_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("UPDATE comment SET text=@p1 WHERE id=@p2")
                            .bind(&text)
                            .bind(comment_id)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Комментарий с id={} обновлён", comment_id),
                            Err(e) => eprintln!("[ERROR] Не удалось обновить комментарий с id={}: {}", comment_id, e),
                        }
                    },
                    82..=83 => {
                        // println!("82..=83");
                        
                        let max_id: i32 = sqlx::query_scalar("SELECT MAX(id) FROM comment")
                            .fetch_one(&pool).await.unwrap_or(1);
                        let comment_id = (rand::random::<u32>() % max_id as u32) as i32 + 1;

                        let res = sqlx::query("DELETE FROM comment WHERE id=@p1")
                            .bind(comment_id)
                            .execute(&pool).await;
                        match res {
                            Ok(_) => println!("[OK] Комментарий с id={} удалён", comment_id),
                            Err(e) => eprintln!("[ERROR] Не удалось удалить комментарий с id={}: {}", comment_id, e),
                        }
                    },
                    84..=88 => {
                        // println!("84..=88");
                        
                        let total_quests: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM quest")
                            .fetch_one(&pool)
                            .await
                            .unwrap_or(1);

                        if total_quests == 0 {
                            eprintln!("[WARN] Нет квестов в базе");
                            return;
                        }

                        let step = (total_quests / 5).max(1);
                        let index = (rand::random::<u32>() % 5) as i32;
                        let quest_id = index * step + 1;

                        let res = sqlx::query("SELECT id, name, required_level FROM quest WHERE id=@p1")
                            .bind(quest_id)
                            .fetch_one(&pool)
                            .await;

                        match res {
                            Ok(_) => println!("[OK] Квест с id={} выбран (часть 1/5)", quest_id),
                            Err(e) => eprintln!("[ERROR] Не удалось выбрать квест с id={}: {}", quest_id, e),
                        }
                    },
                    _ => {}
                }

                let delay_ms = 100 + (rand::random::<u32>() % 400);
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms as u64)).await;
            }
        });

        handlers.push(handle);
    }

    handlers
}