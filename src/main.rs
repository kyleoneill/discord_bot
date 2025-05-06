pub mod db;
pub mod handler;
pub mod logger;
pub mod models;

use db::Database;
use handler::Handler;

#[macro_use]
extern crate dotenv_codegen;

use serenity::prelude::*;
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() {
    // Initialize db
    let database_url = dotenv!("DATABASE_URL").to_owned();
    let pool = SqlitePool::connect(database_url.as_str())
        .await
        .expect("Failed to connect to databse");
    db::init_db(&pool).await;

    // Get a discord token
    let discord_token = dotenv!("DISCORD_TOKEN").to_owned();

    // Set gateway intents, which decide what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;

    // Create discord client
    let mut client = Client::builder(discord_token.as_str(), intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create Discord client");

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(pool);
    }

    // Listen to events on a single shard
    if let Err(why) = client.start().await {
        println!("Client Error: {:?}", why);
    }
}
