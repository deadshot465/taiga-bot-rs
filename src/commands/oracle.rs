use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::{PersistenceService, Oracle};

const THUMBNAIL_URL: &'static str = "https://cdn.discordapp.com/emojis/701918026164994049.png?v=1";

#[command]
#[aliases("fortune")]
#[description = "Draw an oracle and know the future of something on your mind."]
#[usage = ""]
#[example = ""]
#[bucket = "information"]
pub async fn oracle(context: &Context, msg: &Message) -> CommandResult {
    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let persistence_lock = persistence.read().await;
    let oracles = persistence_lock.oracles.as_ref().unwrap();
    let oracle: Oracle;
    {
        let mut rng = thread_rng();
        oracle = oracles.choose(&mut rng).unwrap().clone();
    }
    drop(persistence_lock);
    drop(data);
    let color = u32::from_str_radix("ff0000", 16)?;

    msg.channel_id.send_message(&context.http, |m| m.embed(|e| {
        e.author(|author| {
            if let Some(url) = msg.author.avatar_url().as_ref() {
                author.icon_url(url.as_str());
            }
            author.name(msg.author.name.as_str())
        })
            .color(Color::from(color))
            .field("No", oracle.no, true)
            .field("Meaning", oracle.meaning.as_str(), true)
            .footer(|f| f.text("Wish you good luck!"))
            .description(oracle.content.as_str())
            .thumbnail(THUMBNAIL_URL)
            .title(oracle.fortune.as_str())
    })).await?;
    Ok(())
}