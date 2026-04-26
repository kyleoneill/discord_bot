use super::Database;

use crate::{
    logger::Logger,
    models::pokemon::{
        pokemon_command_record::PokemonCommandRecord,
        pokemon_ownership_record::{PokemonOwnershipRecord, PokemonOwnershipRecordPK},
    },
};

use sqlx::{Error, SqlitePool};

impl Database {
    pub async fn upsert_ownership_record(
        pool: &SqlitePool,
        ownership_record: &PokemonOwnershipRecordPK,
        current_time: i64,
        discord_username: String,
    ) -> Result<(), Error> {
        // TODO: This function shouldn't take a discord_username, need to figuren out a less hacky way
        // to resolve creating a user if it doesn't exist (like, doing it earlier in the handle flow)

        let discord_id = ownership_record.discord_id.clone();

        let mut tx = pool.begin().await?;

        // Check if the current user exists in the db, create a record for them if not
        if let Err(e) = Database::create_user_if_not_exist(pool, discord_id.as_str(), discord_username.as_str()).await {
            Logger::log(format!("Failed to get or create a user with err: {}", e));
        }

        // Upsert the pokemon ownership record for this user
        match Database::get_ownership_record_for_user(pool, ownership_record).await? {
            Some(existing_record) => {
                // This user already has this pokemon, so we want to increment their count for it
                let new_count = existing_record.number_owned + 1;
                sqlx::query!(
                    "UPDATE pokemon_ownership_record SET number_owned = ? WHERE discord_id = ? AND pokemon_slug = ? AND pokemon_type = ?",
                    new_count,
                    existing_record.discord_id,
                    existing_record.pokemon_slug,
                    existing_record.pokemon_type,
                )
                .execute(&mut *tx)
                .await?;
            }
            None => {
                // This user does not have this pokemon yet, so we want to create a new record
                sqlx::query!(
                    "INSERT INTO pokemon_ownership_record (discord_id, pokemon_slug, number_owned, pokemon_type) VALUES (?, ?, 1, ?)",
                    ownership_record.discord_id,
                    ownership_record.pokemon_slug,
                    ownership_record.pokemon_type,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        // Upsert the command record for this user
        sqlx::query!(
            r#"
            UPDATE pokemon_command_record
            SET last_used_at = ?, times_used = times_used + 1
            WHERE discord_id = ?
            "#,
            current_time,
            discord_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_ownership_record_for_user(
        pool: &SqlitePool,
        primary_key: &PokemonOwnershipRecordPK,
    ) -> Result<Option<PokemonOwnershipRecord>, Error> {
        match sqlx::query_as!(
            PokemonOwnershipRecord,
            r#"
            SELECT *
            FROM pokemon_ownership_record
            WHERE discord_id = ? AND pokemon_slug = ? AND pokemon_type = ?
            "#,
            primary_key.discord_id,
            primary_key.pokemon_slug,
            primary_key.pokemon_type,
        )
        .fetch_one(pool)
        .await
        {
            Ok(record) => Ok(Some(record)),
            Err(e) => match e {
                Error::RowNotFound => Ok(None),
                _ => Err(e),
            },
        }
    }

    pub async fn get_command_record_for_user(pool: &SqlitePool, discord_id: &str) -> Result<Option<PokemonCommandRecord>, Error> {
        match sqlx::query_as!(
            PokemonCommandRecord,
            "SELECT * FROM pokemon_command_record WHERE discord_id = ?",
            discord_id
        )
        .fetch_one(pool)
        .await
        {
            Ok(record) => Ok(Some(record)),
            Err(e) => match e {
                Error::RowNotFound => Ok(None),
                _ => Err(e),
            },
        }
    }
}
