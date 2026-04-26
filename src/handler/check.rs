use crate::db::Database;
use crate::logger::Logger;
use crate::models::social_credit::SocialCredit;
use serenity::all::{Context, Message};
use serenity::builder::{CreateEmbed, CreateMessage};

pub async fn check_credit_for_user(ctx: Context, msg: Message) {
    let user = match msg.mentions.len() {
        0 => msg.author,
        _ => msg.mentions[0].clone(),
    };

    let target_user_id: String = user.id.get().to_string();
    let target_user_name: String = user.name;

    let data_read = ctx.data.read().await;
    let db = data_read
        .get::<Database>()
        .expect("Failed to get database")
        .clone();

    let credit = if let Ok(Some(credit)) =
        Database::get_user_social_credit(&db, target_user_id.as_str()).await
    {
        credit
    } else {
        SocialCredit {
            username: target_user_name,
            positive_credit: 0,
            negative_credit: 0,
            traded_credit: 0,
        }
    };

    let embed = CreateEmbed::new()
        .title(format!("{} Social Credit Score", credit.username))
        .field("Positive: ", credit.positive_credit.to_string(), false)
        .field("Negative: ", credit.negative_credit.to_string(), false)
        .field("Sum: ", credit.sum_score().to_string(), false);

    let builder = CreateMessage::new().embed(embed);
    let msg = msg.channel_id.send_message(&ctx.http, builder).await;
    if let Err(why) = msg {
        Logger::log(format!("Failed to send a message with error: {:?}", why))
    }
}
