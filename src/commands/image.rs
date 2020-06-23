use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::CommandStrings;
use crate::{INTERFACE_SERVICE, get_image};
use std::borrow::Borrow;

#[command]
#[aliases("img")]
#[description = "Get a random image based on a keyword."]
#[usage = "<keyword>"]
#[only_in("guilds")]
#[example = "dog"]
#[bucket = "utilities"]
pub async fn image(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.image;
    }

    let keyword = args.single::<String>();
    let mut result: Vec<u8> = vec![];
    if let Ok(s) = &keyword {
        result = get_image(&s).await?;
    }
    else {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        unsafe {
            if INTERFACE_SERVICE.is_kou {
                return Ok(());
            }
            else {
                result = get_image("burger").await?;
            }
        }
    }

    if result.len() == 0 {
        let error_msg = interface_string.errors["no_result"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        Ok(())
    }
    else {
        let result_msg = interface_string.result.as_str()
            .replace("{keyword}", keyword.unwrap().as_str());
        let files: Vec<(&[u8], &str)> = vec![(result.borrow(), "image.jpg")];
        msg.channel_id.send_files(&context.http, files, |f| f.content(&result_msg)).await?;
        Ok(())
    }
}