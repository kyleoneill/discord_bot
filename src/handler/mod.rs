mod check;

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
            return
        }

        // Process a message as a command if it begins with COMMAND_DELIMITER
        if let Some(first_char) = msg.content.chars().next() {
            if first_char == COMMAND_DELIMITER {
                let discord_user = msg.author.name.clone();
                match msg.content.split_whitespace().next() {
                    Some(segment) => match segment {
                        "!check" => check::check_credit_for_user(ctx, msg).await,
                        "!credit" => Logger::log("TODO: Credit"),
                        "!leaderboard" => Logger::log("TODO: Leaderboard"),
                        _ => Logger::log(format!(
                            "User {} tried to use command {}",
                            discord_user, segment
                        )),
                    },
                    None => {
                        Logger::log("Did not get message content when trying to match a command")
                    }
                }
            }
        }
        /*
           - !check @person
               - Checks their score (positive, negative, traded, sum total)
           - !credit @person
               - Gives score to somebody, cannot put you in debt. Must have the given amount as minimum
           - !leaderboard
               - Show the current leaderboard
           - Every friday, show leaderboard?
        */
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let data_read = ctx.data.read().await;
        let db = data_read
            .get::<Database>()
            .expect("Failed to get database")
            .clone();

        // Check what kind of reaction was added, if it wasn't one we track then exit early
        let react_type: ReactType = match add_reaction.emoji {
            ReactionType::Custom {
                name: Some(ref emoji_name),
                ..
            } => match emoji_name.as_str() {
                "positivefriend" => ReactType::AddScore,
                "negativefriend" => ReactType::SubtractScore,
                _ => return,
            },
            _ => return,
        };

        if let Ok(message) = add_reaction.message(ctx.http).await {
            let discord_username = message.author.name;

            // If somebody is reacting to their own message, do not count the score
            if let Some(react_user) = add_reaction.member {
                if discord_username == react_user.user.name {
                    return
                }
            }

            // Verify that the user exists and has a record for social credit
            match Database::create_user_if_not_exist(&db, discord_username.as_str()).await {
                Ok(_) => (),
                Err(e) => {
                    Logger::log(e.as_str());
                    return;
                }
            }

            // Add or deduct credit based on which react we got
            let res = match react_type {
                ReactType::AddScore => {
                    match Database::add_credit_score(&db, discord_username.as_str()).await {
                        Ok(()) => {
                            format!("Added social credit for user {}", discord_username.as_str())
                        }
                        Err(_) => format!(
                            "Failed to add social credit for user {}",
                            discord_username.as_str()
                        ),
                    }
                }
                ReactType::SubtractScore => {
                    match Database::subtract_credit_score(&db, discord_username.as_str()).await {
                        Ok(()) => format!(
                            "Subtracted social credit for user {}",
                            discord_username.as_str()
                        ),
                        Err(_) => format!(
                            "Failed to subtract social credit for user {}",
                            discord_username.as_str()
                        ),
                    }
                }
            };

            Logger::log(res);
        }
    }
}
