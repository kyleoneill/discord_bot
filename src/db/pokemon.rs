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
        current_command_record: Option<PokemonCommandRecord>,
        discord_username: String,
    ) -> Result<(), Error> {
        // TODO: This function shouldn't take a discord_username, need to figuren out a less hacky way
        // to resolve creating a user if it doesn't exist (like, doing it earlier in the handle flow)

        let mut tx = pool.begin().await?;

        // Check if the current user exists in the db, create a record for them if not
        if let Err(e) = Database::create_user_if_not_exist(
            pool,
            ownership_record.discord_id.as_str(),
            discord_username.as_str(),
        )
        .await
        {
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
                ).execute(&mut *tx).await?;
            }
            None => {
                // This user does not have this pokemon yet, so we want to create a new record
                sqlx::query!(
                    "INSERT INTO pokemon_ownership_record (discord_id, pokemon_slug, number_owned, pokemon_type) VALUES (?, ?, 1, ?)",
                    ownership_record.discord_id,
                    ownership_record.pokemon_slug,
                    ownership_record.pokemon_type,
                ).execute(&mut *tx).await?;
            }
        }

        // Upsert the command record for this user
        match current_command_record {
            Some(record) => {
                let new_times_used = record.times_used + 1;
                sqlx::query!(
                    "UPDATE pokemon_command_record SET last_used_at = ?, last_used_at = ? WHERE discord_id = ?",
                    new_times_used,
                    current_time,
                    ownership_record.discord_id,
                ).execute(&mut *tx).await?;
            }
            None => {
                sqlx::query!(
                    "INSERT INTO pokemon_command_record (discord_id, last_used_at, times_used) VALUES (?, ?, 1)",
                    ownership_record.discord_id,
                    current_time,
                ).execute(&mut *tx).await?;
            }
        }
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

    pub async fn get_command_record_for_user(
        pool: &SqlitePool,
        discord_id: &str,
    ) -> Result<Option<PokemonCommandRecord>, Error> {
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
