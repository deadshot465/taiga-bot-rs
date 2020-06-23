use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::{PERSISTENCE_STORAGE, INTERFACE_SERVICE};
use crate::shared::{Character, CommandStrings};

#[command]
#[aliases("v")]
#[description = "Tells you your next valentine."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "information"]
pub async fn valentine(context: &Context, msg: &Message) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.valentine;
    }
    let valentine = get_valentine().await;
    let is_keitaro = get_first_name(valentine.name.as_str()) == "Keitaro";
    let prefix_suffix = if is_keitaro {
        "~~"
    }
    else {
        ""
    };

    let footer = if is_keitaro {
        interface_string.infos["keitaro_footer"].clone()
    }
    else {
        interface_string.infos["normal_footer"].clone()
            .replace("{firstName}", get_first_name(valentine.name.as_str()))
    };

    let valentine_name = format!("{}Your valentine is {}{}", prefix_suffix, valentine.name.as_str(), prefix_suffix);
    let color = u32::from_str_radix(&valentine.color.as_str(), 16).unwrap() as i32;

    if is_keitaro {
        let message = interface_string.infos["keitaro_header"].as_str();
        msg.channel_id.say(&context.http, message).await?;
    }

    msg.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.author(|a| {
                if let Some(url) = msg.author.avatar_url().as_ref() {
                    a.icon_url(url);
                }
                a.name(&msg.author.name)
            })
                .color(Color::from(color))
                .description(format!("{}{}{}", prefix_suffix, valentine.description.as_str(), prefix_suffix))
                .field("Age", valentine.age, true)
                .field("Birthday", valentine.birthday.as_str(), true)
                .field("Animal Motif", valentine.animal.as_str(), true)
                .footer(|f| f.text(&footer))
                .thumbnail(get_emote_url(valentine.emote_id.as_str()))
                .title(valentine_name.as_str())
        })
    }).await?;

    Ok(())
}

async fn get_valentine() -> &'static Character {
    unsafe {
        let valentines = PERSISTENCE_STORAGE.valentines.as_ref().unwrap();
        &valentines[thread_rng().gen_range(0, valentines.len())]
    }
}

fn get_first_name(name: &str) -> &str {
    let first_name: Vec<&str> = name.split(' ').collect();
    first_name[0]
}

fn get_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{}.png?v=1", emote_id)
}