use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;

const KOU_EMOTE_URL: &'static str = "https://cdn.discordapp.com/emojis/700119260394946620.png";
const HIRO_EMOTE_URL: &'static str = "https://cdn.discordapp.com/emojis/704022326412443658.png";
const KOU_NAME: &'static str = "Minamoto Kou";
const HIRO_NAME: &'static str = "Akiba Hiro";

#[command]
pub async fn ship(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg1 = args.single::<String>()?;
    let arg2 = args.single::<String>()?;
    let lower_arg1 = arg1.to_lowercase();
    let lower_arg2 = arg2.to_lowercase();

    if lower_arg1.contains("kou") && lower_arg2.contains("hiro") {

    }

    Ok(())
}

async fn ship_secret_romance<'a>(context: &Context, msg: &Message, arg1: &'a str, arg2: &'a str) -> CommandResult {
    let (score, message) = (10000, format!("What are you talking about? {} and {} of course are the cutest two!", arg1, arg2));

}