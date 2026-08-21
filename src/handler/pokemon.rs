use crate::db::Database;
use crate::logger::Logger;
use crate::models::pokemon::{PokemonData, pokemon_info::Pokemon, pokemon_ownership_record::PokemonOwnershipRecordPK, roll_for_type};
use crate::util::{current_time_unix_epoch, seconds_to_human_readable};

use rand::prelude::IndexedRandom;
use serenity::all::{Context, CreateEmbedFooter, Message, Timestamp};
use serenity::builder::{CreateEmbed, CreateMessage};

const FOOTERS: [(&str, &str); 9] = [
    ("Get squirted on", "https://play.pokemonshowdown.com/sprites/itemicons/squirtbottle.png"),
    ("You've got mail", "https://play.pokemonshowdown.com/sprites/itemicons/air-mail.png"),
    (
        "Bought with mom's credit card",
        "https://play.pokemonshowdown.com/sprites/itemicons/blue-card.png",
    ),
    ("CHAOS CONTROL", "https://play.pokemonshowdown.com/sprites/itemicons/bug-gem.png"),
    ("Pills here", "https://play.pokemonshowdown.com/sprites/itemicons/calcium.png"),
    (
        "Lord Helix was here",
        "https://play.pokemonshowdown.com/sprites/itemicons/helix-fossil.png",
    ),
    ("Listen to my mixtape", "https://play.pokemonshowdown.com/sprites/itemicons/hm-dragon.png"),
    (
        "There's a bottom healing on everyone!",
        "https://play.pokemonshowdown.com/sprites/itemicons/hyper-potion.png",
    ),
    (
        "You fell for my master bait",
        "https://play.pokemonshowdown.com/sprites/itemicons/master-ball.png",
    ),
];

pub async fn handle_pokemon_command(ctx: Context, msg: Message) {
    let mut split = msg.content.split_whitespace();
    split.next().expect("The message must have begun with a 'pokemon' command");
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
        let pokemon_data = data_read.get::<PokemonData>().expect("State data must have PokemonData");
        pokemon_data.get_random_pokemon()
    };

    let data_read = ctx.data.read().await;
    let db = data_read.get::<Database>().expect("Failed to get database").clone();

    let discord_id = msg.author.id.get().to_string();
    let discord_username = msg.author.name.to_owned();

    let ownership_record = PokemonOwnershipRecordPK {
        discord_id: discord_id.clone(),
        pokemon_slug: random_pokemon.slug.clone(),
        pokemon_type: pokemon_type.clone(),
    };

    // Check if the user has a command record and if they used the command too recently
    let current_time = current_time_unix_epoch();
    match Database::get_command_record_for_user(&db, discord_id.as_str()).await {
        Ok(maybe_record) => {
            if let Some(command_record) = maybe_record
                && let Err(time_remaining) = command_record.enough_time_since_timestamp(current_time)
            {
                // The user has used this command too recently
                let time_remaining = seconds_to_human_readable(time_remaining);
                let builder = CreateMessage::new().content(format!(
                    ":x: You need to wait another {} before catching another pokemon.",
                    time_remaining
                ));
                if let Err(e) = msg.channel_id.send_message(&ctx.http, builder).await {
                    Logger::log(format!("Failed to respond to pokemon command with err: {}", e));
                }
                return;
            }
        }
        Err(e) => {
            Logger::log(format!("Failed to check for a pokemon command record with error: {}", e));
            // TODO: Return an error to the user here
            return;
        }
    };

    if let Err(e) = Database::upsert_ownership_record(&db, &ownership_record, current_time, discord_username).await {
        Logger::log(format!("Failed to upsert pokemon ownership record with err: {}", e));
        return;
    };

    // Generate a discord message to return to the user
    let display_url = pokemon_type.get_link_for_type(&random_pokemon);
    let display_text = format!(
        "<@{}>, {}",
        msg.author.id.clone(),
        pokemon_type.get_display_text_for_type(random_pokemon.name.to_owned())
    );
    let embed_color = random_pokemon.rarity.get_color_for_embed();

    let footer = {
        // This is scoped into a block so rng is dropped before reaching an await, which causes a compiler error
        // as it does not impl Send

        // Choose a footer text and image
        let mut rng = rand::rng();
        let (footer_text, footer_image) = FOOTERS.choose(&mut rng).expect("const slice will always contain data");
        CreateEmbedFooter::new(*footer_text).icon_url(*footer_image)
    };

    // TODO: have footer image dependent on social credit score
    // High credit = vanity ball (premier, moon, etc)
    // Low credit = Generic
    // Negative credit = Something else?
    // TODO: Add a field to the pokemon ownership record of what pokeball was used to catch it, matching the pokeball icon used here
    // TODO: Add an inventory so people can use a specific pokeball on a catch? The ball can maybe influence what pokemon is caught?
    // TODO: Add support for forms (unown, alcremie, etc) - will need to make sure that primary key storage and url generation work for this

    let pokemon_type: String = random_pokemon.types.join(" / ");
    let height = format!("{}m", random_pokemon.height);
    let weight = format!("{}kg", random_pokemon.weight);

    let embed = CreateEmbed::new()
        .description(display_text)
        .color(embed_color)
        .image(display_url)
        .footer(footer)
        .timestamp(Timestamp::now())
        .field("Type", pokemon_type, true)
        .field("Height", height, true)
        .field("Weight", weight, true)
        .field("Base Stats", random_pokemon.base_stats.to_string(), false);

    let builder = CreateMessage::new().embed(embed);
    if let Err(e) = msg.channel_id.send_message(&ctx.http, builder).await {
        Logger::log(format!("Failed to respond to pokemon command with err: {}", e));
    };
}
