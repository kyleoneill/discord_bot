pub mod leaderboard;

use crate::models::social_credit::SocialCredit;

use serenity::prelude::TypeMapKey;
use sqlx::{Error, SqlitePool};

pub async fn init_db(pool: &SqlitePool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Failed to run migrations");
}

pub struct Database;

impl TypeMapKey for Database {
    type Value = SqlitePool;
}

impl Database {
    pub async fn create_user_if_not_exist(
        pool: &SqlitePool,
        discord_username: &str,
    ) -> Result<(), String> {
        match Database::get_user_social_credit(pool, discord_username).await {
            Ok(_user_credit) => Ok(()),
            Err(e) => match e {
                Error::RowNotFound => match Database::add_user(pool, discord_username).await {
                    Ok(()) => Ok(()),
                    Err(_) => Err(
                        "Failed to add user to db when handling social credit retrieval"
                            .to_string(),
                    ),
                },
                _ => Err("Unhandled error when getting a user social credit".to_string()),
            },
        }
    }

    pub async fn get_user_social_credit(
        pool: &SqlitePool,
        discord_username: &str,
    ) -> Result<Option<SocialCredit>, Error> {
        match sqlx::query_as!(
            SocialCredit,
            "SELECT * FROM social_credit WHERE username = ?",
            discord_username
        )
        .fetch_one(pool)
        .await
        {
            Ok(res) => Ok(Some(res)),
            Err(e) => Err(e),
        }
    }

    pub async fn add_user(pool: &SqlitePool, discord_username: &str) -> Result<(), Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            r#"
            INSERT OR IGNORE INTO users (discord_username)
            VALUES (?1);
            "#,
        )
        .bind(discord_username)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT OR IGNORE INTO social_credit (username)
            VALUES (?1);
            "#,
        )
        .bind(discord_username)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn add_credit_score(pool: &SqlitePool, discord_username: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE social_credit
            SET positive_credit = positive_credit + 1
            WHERE username = ?1
            "#,
            discord_username
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn subtract_credit_score(
        pool: &SqlitePool,
        discord_username: &str,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE social_credit
            SET negative_credit = negative_credit + 1
            WHERE username = ?1
            "#,
            discord_username
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
