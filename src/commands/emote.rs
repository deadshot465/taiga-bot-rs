use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;

const KEK_LINK: &'static str = "https://cdn.discordapp.com/emojis/730239295155077251.png";
const PENSIVE_LINK: &'static str = "https://cdn.discordapp.com/emojis/709416604269543514.gif";
const BAKUGO_LINK: &'static str = "https://cdn.discordapp.com/emojis/703488149728526357.png";
const YURI_LINK: &'static str = "https://cdn.discordapp.com/emojis/540259158155329601.gif";

#[command]
#[description = ":kek:"]
#[usage = ""]
#[example = ""]
#[bucket = "fun"]
pub async fn kek(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<u8>();
    if let Ok(count) = arg {
        let mut link = String::from("<:kek:730239295155077251>");
        for _ in 1..count {
            link += " <:kek:730239295155077251>";
        }
        msg.channel_id.say(&context.http, &link)
            .await?;
    }
    else {
        msg.channel_id.say(&context.http, KEK_LINK)
            .await?;
    }
    Ok(())
}

#[command]
#[description = ":PensiveWeird:"]
#[usage = ""]
#[example = ""]
#[bucket = "fun"]
pub async fn pensive(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<u8>();
    if let Ok(count) = arg {
        let mut link = String::from("<a:PensiveWeird:716110336910032897>");
        for _ in 1..count {
            link += " <a:PensiveWeird:716110336910032897>";
        }
        msg.channel_id.say(&context.http, &link)
            .await?;
    }
    else {
        msg.channel_id.say(&context.http, PENSIVE_LINK)
            .await?;
    }
    Ok(())
}

#[command]
#[description = ":PensiveWeird:"]
#[usage = ""]
#[example = ""]
#[bucket = "fun"]
pub async fn yurishake(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<u8>();
    if let Ok(count) = arg {
        let mut link = String::from("<a:yuri_shake:735400312126570526>");
        for _ in 1..count {
            link += " <a:yuri_shake:735400312126570526>";
        }
        msg.channel_id.say(&context.http, &link)
            .await?;
    }
    else {
        msg.channel_id.say(&context.http, YURI_LINK)
            .await?;
    }
    Ok(())
}

#[command]
#[description = ":PensiveWeird:"]
#[usage = ""]
#[example = ""]
#[bucket = "fun"]
pub async fn bakugo(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<u8>();
    if let Ok(count) = arg {
        let mut link = String::from("<:BakugoLaugh:703488149728526357>");
        for _ in 1..count {
            link += " <:BakugoLaugh:703488149728526357>";
        }
        msg.channel_id.say(&context.http, &link)
            .await?;
    }
    else {
        msg.channel_id.say(&context.http, BAKUGO_LINK)
            .await?;
    }
    Ok(())
}