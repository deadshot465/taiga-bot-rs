use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use owoify_rs::{Owoifiable, OwoifyLevel};
use crate::shared::CommandStrings;
use crate::INTERFACE_SERVICE;

#[command]
#[description = "This command will owoify your text."]
#[usage = "<text> or owoify <soft|medium|hard> <text>"]
#[example = "hard This is the message to owo! The cutest one!"]
#[bucket = "fun"]
pub async fn owoify(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut mode: OwoifyLevel = OwoifyLevel::Owo;
    let first_arg = args.single::<String>();
    let mut use_default = false;
    if let Ok(m) = &first_arg {
        mode = match m.to_lowercase().trim() {
            "soft" => OwoifyLevel::Owo,
            "medium" => OwoifyLevel::Uwu,
            "hard" => OwoifyLevel::Uvu,
            _ => {
                use_default = true;
                OwoifyLevel::Owo
            }
        };
    }

    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.owoify;
    }

    let remains = args.remains();
    if let Some(message) = remains {
        if message.is_empty() || message.len() == 0 {
            let error_msg = interface_string.errors["length_too_short"].as_str();
            msg.channel_id.say(&context.http, error_msg).await?;
            return Ok(());
        }
        else if message.len() > 1000 {
            let error_msg = interface_string.errors["length_too_long"].as_str();
            msg.channel_id.say(&context.http, error_msg).await?;
            return Ok(());
        }

        let result = if use_default {
            let input = String::from(first_arg.unwrap().as_str()) + " " + message;
            input.owoify(&mode)
                .replace("`", "\\`")
                .replace("*", "\\*")
        }
        else {
            message.owoify(&mode)
                .replace("`", "\\`")
                .replace("*", "\\*")
        };
        let header_message = interface_string.result.as_str()
            .replace("{author}", msg.author.name.as_str())
            .replace("{text}", &result);
        msg.channel_id.say(&context.http, &header_message).await?;
    }
    else {
        if use_default {
            let message = first_arg.unwrap().as_str().owoify(&mode)
                .replace("`", "\\`")
                .replace("*", "\\*");
            let header_message = interface_string.result.as_str()
                .replace("{author}", msg.author.name.as_str())
                .replace("{text}", &message);
            msg.channel_id.say(&context.http, &header_message).await?;
        }
        else {
            let error_msg = interface_string.errors["length_too_short"].as_str();
            msg.channel_id.say(&context.http, error_msg).await?;
        }
    }
    Ok(())
}