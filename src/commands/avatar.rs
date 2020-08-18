use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::CommandStrings;
use crate::{INTERFACE_SERVICE, search_user};

#[command]
#[aliases("pfp")]
#[description = "Get avatar/profile image of yourself or another user."]
#[usage = "or avatar <username>"]
#[example = "Kou"]
async fn avatar(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.avatar;
    }

    let query = args.remains();
    let username: String;
    let url: Option<String>;
    let pronoun: String;

    if let Some(s) = query {
        let guild = context.cache.guild(msg.guild_id.expect("Failed to get guild information from the message"))
            .await
            .expect("Failed to retrieve guild information.");
        let users = search_user(context, &guild, s).await?;
        if users.is_empty() {
            msg.channel_id.say(&context.http, interface_string.errors["no_result"].as_str()).await?;
            return Ok(());
        }
        username = users[0].nick.clone().unwrap_or(users[0].user.name.clone());
        url = users[0].user.avatar_url();
        pronoun = String::from("This user");
    }
    else {
        username = msg
            .author
            .nick_in(&context.http, msg.guild_id.expect("Failed to get guild information from the message"))
            .await
            .unwrap_or(msg.author.name.clone());
        url = msg.author.avatar_url();
        pronoun = String::from("You");
    }

    if url.is_none() {
        msg.channel_id.say(&context.http, format!("{} don't have an avatar image!", &pronoun))
            .await?;
        return Ok(());
    }

    msg.channel_id.send_message(&context.http, |m| m.embed(|e| {
        e.title(&username);
        let result_string = interface_string
            .result
            .replace("{username}", &username)
            .replace("{url}", url.as_ref().unwrap());
        e.description(&result_string);
        e.image(url.as_ref().unwrap());
        e
    })).await?;
    Ok(())
}