use crate::db::Database;
use crate::logger::Logger;
use crate::models::leaderboard::LeaderboardType;

use serenity::all::{Context, Message};
use serenity::builder::{CreateEmbed, CreateMessage};

pub async fn get_leaderboard(ctx: Context, msg: Message) {
    // TODO: Should probably have a more standardized way to handle/validate command args
    // TODO: Should have a `help` or something here to explain the possible options and what they do
    let command_segments: Vec<&str> = msg.content.split_whitespace().collect();
    let leaderboard_type: LeaderboardType = match command_segments.len() {
        // If the user did not specify which leaderboard, default to the sum
        1 => LeaderboardType::Sum,
        _ => match command_segments[1] {
            "sum" => LeaderboardType::Sum,
            "positive" => LeaderboardType::Positive,
            "negative" => LeaderboardType::Negative,
            "total" => LeaderboardType::Total,
            _ => LeaderboardType::Sum,
        },
    };

    let data_read = ctx.data.read().await;
    let db = data_read
        .get::<Database>()
        .expect("Failed to get database")
        .clone();

    match Database::get_leaderboard(&db, &leaderboard_type).await {
        Ok(res) => {
            let send_res = if res.is_empty() {
                msg.channel_id
                    .send_message(
                        &ctx.http,
                        CreateMessage::new()
                            .content("There are no entries yet to rank on a leaderboard."),
                    )
                    .await
            } else {
                let mut places = String::new();
                let mut users = String::new();
                let mut scores = String::new();

                for (pos, entry) in res.iter().enumerate() {
                    match pos {
                        0 => places += ":first_place:\n",
                        1 => places += ":second_place:\n",
                        2 => places += ":third_place:\n",
                        _ => places += &format!("{}\n", pos + 1),
                    }
                    users += &format!("{}\n", entry.username);
                    scores += &format!("{}\n", entry.vote_count);
                }

                let embed = CreateEmbed::new()
                    .title(format!("{} Leaderboard", leaderboard_type))
                    .field("Place", places, true)
                    .field("User", users, true)
                    .field("Score", scores, true);

                let builder = CreateMessage::new().embed(embed);
                msg.channel_id.send_message(&ctx.http, builder).await
            };
            if let Err(why) = send_res {
                Logger::log(format!("Failed to send a message with error: {:?}", why))
            }
        }
        Err(e) => Logger::log(format!("Failed to get leaderboard with error: {:?}", e)),
    }
}
