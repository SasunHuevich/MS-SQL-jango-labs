use sqlx::{Pool, Mssql};
use fake::faker::internet::en::{SafeEmail, Username, Password};
use fake::Fake;
use rand::{Rng, prelude::IndexedRandom};
use chrono::{Utc, Duration};

pub async fn seed_users(
    pool: &Pool<Mssql>,
    n: usize,
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

    for offset in (0..n).step_by(BATCH_SIZE) {
        let mut tx = pool.begin().await?;

        for _ in 0..BATCH_SIZE.min(n - offset) {
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
            (offset + BATCH_SIZE).min(n),
            n
        );
    }

    Ok(())
}