use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::{get_image, InterfaceService};
use std::borrow::Borrow;
use std::sync::Arc;

#[command]
#[aliases("img")]
#[description = "Get a random image based on a keyword."]
#[usage = "<keyword>"]
#[only_in("guilds")]
#[example = "dog"]
#[bucket = "utilities"]
pub async fn image(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = context.data.read().await;
    let interface = data.get::<InterfaceService>().unwrap();
    let _interface = Arc::clone(interface);
    drop(data);
    let interface_lock = _interface.lock().await;
    let interface = interface_lock.interface_strings.as_ref().unwrap();
    let interface_string = &interface.image;

    let keyword = args.single::<String>();
    let result: Vec<u8>;
    if let Ok(s) = &keyword {
        result = get_image(&s).await?;
    }
    else {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        let data = context.data.read().await;
        let interface = data.get::<InterfaceService>().unwrap();
        let interface_lock = interface.lock().await;
        let is_kou = interface_lock.is_kou;
        drop(interface_lock);
        drop(data);
        if is_kou {
            return Ok(());
        }
        else {
            result = get_image("burger").await?;
        }
    }

    if result.len() == 0 {
        let error_msg = interface_string.errors["no_result"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        drop(interface_lock);
        Ok(())
    }
    else {
        let result_msg = interface_string.result.as_str()
            .replace("{keyword}", keyword.unwrap().as_str());
        drop(interface_lock);
        let files: Vec<(&[u8], &str)> = vec![(result.borrow(), "image.jpg")];
        msg.channel_id.send_files(&context.http, files, |f| f.content(&result_msg)).await?;
        Ok(())
    }
}