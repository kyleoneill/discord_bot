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
    ) -> Result<(), Error> {
        let mut tx = pool.begin().await?;

        // Check if the current user exists in the db, create a record for them if not
        if let Err(e) =
            Database::create_user_if_not_exist(pool, &ownership_record.username.as_str()).await
        {
            Logger::log(format!("Failed to get or create a user with err: {}", e));
        }

        // Upsert the pokemon ownership record for this user
        match Database::get_ownership_record_for_user(pool, ownership_record).await? {
            Some(existing_record) => {
                // This user already has this pokemon, so we want to increment their count for it
                let new_count = existing_record.number_owned + 1;
                sqlx::query!(
                    "UPDATE pokemon_ownership_record SET number_owned = ? WHERE username = ? AND pokemon_name = ? AND shiny = ? AND april_fools = ?",
                    new_count,
                    existing_record.username,
                    existing_record.pokemon_name,
                    existing_record.shiny,
                    existing_record.april_fools,
                ).execute(&mut *tx).await?;
            }
            None => {
                // This user does not have this pokemon yet, so we want to create a new record
                sqlx::query!(
                    "INSERT INTO pokemon_ownership_record (username, pokemon_name, number_owned, shiny, april_fools) VALUES (?, ?, 1, ?, ?)",
                    ownership_record.username,
                    ownership_record.pokemon_name,
                    ownership_record.shiny,
                    ownership_record.april_fools,
                ).execute(&mut *tx).await?;
            }
        }

        // Upsert the command record for this user
        match current_command_record {
            Some(record) => {
                let new_times_used = record.times_used + 1;
                sqlx::query!(
                    "UPDATE pokemon_command_record SET last_used_at = ?, last_used_at = ? WHERE username = ?",
                    new_times_used,
                    current_time,
                    ownership_record.username,
                ).execute(&mut *tx).await?;
            }
            None => {
                sqlx::query!(
                    "INSERT INTO pokemon_command_record (username, last_used_at, times_used) VALUES (?, ?, 1)",
                    ownership_record.username,
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
            "SELECT * FROM pokemon_ownership_record WHERE username = ? AND pokemon_name = ? AND shiny = ? AND april_fools = ?",
            primary_key.username,
            primary_key.pokemon_name,
            primary_key.shiny,
            primary_key.april_fools
        ).fetch_one(pool).await {
            Ok(record) => Ok(Some(record)),
            Err(e) => {
                match e {
                    Error::RowNotFound => return Ok(None),
                    _ => return Err(e)
                }
            }
        }
    }

    pub async fn get_command_record_for_user(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<Option<PokemonCommandRecord>, Error> {
        match sqlx::query_as!(
            PokemonCommandRecord,
            "SELECT * FROM pokemon_command_record WHERE username = ?",
            username
        )
        .fetch_one(pool)
        .await
        {
            Ok(record) => Ok(Some(record)),
            Err(e) => match e {
                Error::RowNotFound => return Ok(None),
                _ => return Err(e),
            },
        }
    }
}
