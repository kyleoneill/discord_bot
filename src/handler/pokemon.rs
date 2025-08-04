use crate::db::Database;
use crate::logger::Logger;
use crate::models::pokemon::{
    PokemonData,
    pokemon_ownership_record::{PokemonOwnershipRecord, PokemonOwnershipRecordPK},
    roll_for_april_fools, roll_for_shiny,
};
use crate::util::{current_time_unix_epoch, seconds_to_human_readable};

use serenity::all::{Context, Message};
use serenity::builder::{CreateEmbed, CreateMessage};

pub async fn handle_pokemon_command(ctx: Context, msg: Message) {
    let mut split = msg.content.split_whitespace();
    split
        .next()
        .expect("The message must have begun with a 'pokemon' command");
    match split.next() {
        Some(sub_command) => {
            match sub_command {
                // TODO: Future subcommands

                // TODO: pokemon check {name} -> check if I have a pokemon

                // If we get an unsupported sub-command, just default
                _ => get_random_pokemon(ctx, msg).await,
            }
        }
        None => get_random_pokemon(ctx, msg).await,
    }
}

pub async fn get_random_pokemon(ctx: Context, msg: Message) {
    let is_april_fools = roll_for_april_fools();
    let is_shiny = {
        // Pokemon cannot be both april fools and shiny
        if is_april_fools {
            false
        } else {
            roll_for_shiny()
        }
    };

    let random_pokemon = {
        let data_read = ctx.data.read().await;
        let pokemon_data = data_read
            .get::<PokemonData>()
            .expect("State data must have PokemonData");
        match is_april_fools {
            true => pokemon_data.get_random_april_fools_pokemon(),
            false => pokemon_data.get_random_pokemon(),
        }
    };

    let data_read = ctx.data.read().await;
    let db = data_read
        .get::<Database>()
        .expect("Failed to get database")
        .clone();

    let ownership_record = PokemonOwnershipRecordPK {
        username: msg.author.name.clone(),
        pokemon_name: random_pokemon.clone(),
        shiny: is_shiny,
        april_fools: is_april_fools,
    };

    // Check if the user has a command record and if they used the command too recently
    let current_time = current_time_unix_epoch();
    let current_command_record =
        match Database::get_command_record_for_user(&db, msg.author.name.as_str()).await {
            Ok(maybe_record) => {
                match maybe_record {
                    Some(command_record) => {
                        if let Err(time_remaining) =
                            command_record.enough_time_since_timestamp(current_time)
                        {
                            // The user has used this command too recently
                            let time_remaining = seconds_to_human_readable(time_remaining);
                            let builder = CreateMessage::new().content(format!(
                                ":x: You need to wait another {} before catching another pokemon.",
                                time_remaining
                            ));
                            if let Err(e) = msg.channel_id.send_message(&ctx.http, builder).await {
                                Logger::log(format!(
                                    "Failed to respond to pokemon command with err: {}",
                                    e
                                ));
                            }
                            return;
                        }
                        Some(command_record)
                    }
                    None => None,
                }
            }
            Err(e) => {
                Logger::log(format!(
                    "Failed to check for a pokemon command record with error: {}",
                    e
                ));
                return;
            }
        };

    if let Err(e) = Database::upsert_ownership_record(
        &db,
        &ownership_record,
        current_time,
        current_command_record,
    )
    .await
    {
        Logger::log(format!(
            "Failed to upsert pokemon ownership record with err: {}",
            e
        ));
        return;
    };

    // TODO: MAKE THIS A NICE LOOKING EMBED
    let builder = CreateMessage::new().content(format!("You got {}", random_pokemon));
    if let Err(e) = msg.channel_id.send_message(&ctx.http, builder).await {
        Logger::log(format!(
            "Failed to respond to pokemon command with err: {}",
            e
        ));
    };
}
