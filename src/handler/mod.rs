mod check;
mod leaderboard;
mod pokemon;

use crate::db::Database;
use crate::logger::Logger;
use serenity::all::{Context, EventHandler, Message, Reaction, ReactionType};
use serenity::async_trait;

const COMMAND_DELIMITER: char = '!';

enum ReactType {
    AddScore,
    SubtractScore,
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Do not process a message if it was sent by a bot
        if msg.author.bot {
            return;
        }

        // Process a message as a command if it begins with COMMAND_DELIMITER
        if let Some(first_char) = msg.content.chars().next()
            && first_char == COMMAND_DELIMITER
        {
            // This must be a string as sqlite does not support u64
            let discord_user_id = msg.author.id.get().to_string();
            match msg.content.split_whitespace().next() {
                Some(segment) => {
                    // Strip out the command prefix and make the command case-insensitive
                    let command = segment[1..].to_lowercase();
                    match command.as_str() {
                        "check" => check::check_credit_for_user(ctx, msg).await,
                        "credit" => Logger::log("TODO: Credit"),
                        "leaderboard" => leaderboard::get_leaderboard(ctx, msg).await,
                        "pokemon" => pokemon::handle_pokemon_command(ctx, msg).await,
                        _ => Logger::log(format!("User {} tried to use command {}", discord_user_id, segment)),
                    }
                }
                None => Logger::log("Did not get message content when trying to match a command"),
            }
        }
        /*
        TODO:
           - !credit @person
               - Gives score to somebody, cannot put you in debt. Must have the given amount as minimum
        */
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let data_read = ctx.data.read().await;
        let db = data_read.get::<Database>().expect("Failed to get database").clone();

        // Check what kind of reaction was added, if it wasn't one we track then exit early
        let react_type: ReactType = match add_reaction.emoji {
            ReactionType::Custom {
                name: Some(ref emoji_name), ..
            } => match emoji_name.as_str() {
                "positivefriend" => ReactType::AddScore,
                "negativefriend" => ReactType::SubtractScore,
                _ => return,
            },
            _ => return,
        };

        if let Ok(message) = add_reaction.message(ctx.http).await {
            // This must be a string as sqlite does not support u64
            let discord_user_id = message.author.id.get().to_string();
            let discord_username = message.author.name.clone();

            // If somebody is reacting to their own message, do not count the score
            if let Some(react_user) = add_reaction.member
                && discord_user_id == react_user.user.id.get().to_string()
            {
                return;
            }

            // Verify that the user exists and has a record for social credit
            match Database::create_user_if_not_exist(&db, discord_user_id.as_str(), discord_username.as_str()).await {
                Ok(_) => (),
                Err(e) => {
                    Logger::log(e.as_str());
                    return;
                }
            }

            // Add or deduct credit based on which react we got
            let res = match react_type {
                ReactType::AddScore => match Database::add_credit_score(&db, discord_user_id.as_str()).await {
                    Ok(()) => {
                        format!("Added social credit for user {}", discord_user_id)
                    }
                    Err(_) => {
                        format!("Failed to add social credit for user {}", discord_user_id)
                    }
                },
                ReactType::SubtractScore => match Database::subtract_credit_score(&db, discord_user_id.as_str()).await {
                    Ok(()) => format!("Subtracted social credit for user {}", discord_user_id),
                    Err(_) => format!("Failed to subtract social credit for user {}", discord_user_id),
                },
            };
            Logger::log(res);
        }
    }
}
