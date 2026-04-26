use crate::db::Database;
use crate::logger::Logger;
use crate::models::pokemon::{
    PokemonData, pokemon_info::Pokemon, pokemon_ownership_record::PokemonOwnershipRecordPK,
    roll_for_type,
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
            #[allow(clippy::match_single_binding)]
            match sub_command {
                // TODO: Future subcommands

                // TODO: pokemon check {name} -> check if I have a pokemon by name

                // TODO: List my mythicals, legendaries, etc

                // TODO: List my pokemon

                // TODO: Leaderboard? Who has the most pokemon / legendaries / mystics?

                // If we get an unsupported sub-command, just default
                _ => catch_random_pokemon(ctx, msg).await,
            }
        }
        None => catch_random_pokemon(ctx, msg).await,
    }
}

pub async fn catch_random_pokemon(ctx: Context, msg: Message) {
    // TODO: Need to return a message to the user in the case that this fails

    let pokemon_type = roll_for_type();

    let random_pokemon: Pokemon = {
        let data_read = ctx.data.read().await;
        let pokemon_data = data_read
            .get::<PokemonData>()
            .expect("State data must have PokemonData");
        pokemon_data.get_random_pokemon()
    };

    let data_read = ctx.data.read().await;
    let db = data_read
        .get::<Database>()
        .expect("Failed to get database")
        .clone();

    let discord_id = msg.author.id.get().to_string();
    let discord_username = msg.author.name.to_owned();

    let ownership_record = PokemonOwnershipRecordPK {
        discord_id: discord_id.clone(),
        pokemon_slug: random_pokemon.slug.clone(),
        pokemon_type: pokemon_type.clone(),
    };

    // Check if the user has a command record and if they used the command too recently
    // TODO: Put this in its own function in the db file, will have to refactor to return a specific error to construct a message here if the user
    //       has used the command too recently (separate out our error from a sqlite error)
    let current_time = current_time_unix_epoch();
    let current_command_record =
        match Database::get_command_record_for_user(&db, discord_id.as_str()).await {
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
        discord_username,
    )
    .await
    {
        Logger::log(format!(
            "Failed to upsert pokemon ownership record with err: {}",
            e
        ));
        return;
    };

    let display_url = pokemon_type.get_link_for_type(&random_pokemon);
    let display_text = format!(
        "<@{}>, {}",
        msg.author.id.clone(),
        pokemon_type.get_display_text_for_type(random_pokemon.name.to_owned())
    );
    let embed_color = random_pokemon.rarity.get_color_for_embed();

    let embed = CreateEmbed::new()
        .description(display_text)
        .color(embed_color)
        .image(display_url);

    let builder = CreateMessage::new().embed(embed);
    if let Err(e) = msg.channel_id.send_message(&ctx.http, builder).await {
        Logger::log(format!(
            "Failed to respond to pokemon command with err: {}",
            e
        ));
    };
}
