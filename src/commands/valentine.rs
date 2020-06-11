use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::PERSISTENCE_STORAGE;
use crate::shared::Character;

#[command]
pub fn valentine(context: &mut Context, msg: &Message) -> CommandResult {
    let valentine = get_valentine();
    let is_keitaro = get_first_name(valentine.name.as_str()) == "Keitaro";
    let prefix_suffix = if is_keitaro {
        "~~"
    }
    else {
        ""
    };

    let footer = if is_keitaro {
        "See? Told you Keitaro is my boyfriend. Later loser.".to_string()
    }
    else {
        format!("Don't fret if {} isn't your type. Who knows, maybe it's time for a new favorite.", get_first_name(valentine.name.as_str()))
    };

    let valentine_name = format!("{}Your valentine is {}{}", prefix_suffix, valentine.name.as_str(), prefix_suffix);
    let color = u32::from_str_radix(&valentine.color.as_str(), 16).unwrap() as i32;

    msg.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.author(|a| {
                if let Some(url) = msg.author.avatar.as_ref() {
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
    })?;

    Ok(())
}

fn get_valentine() -> &'static Character {
    let mut rng = thread_rng();
    unsafe {
        let valentines = &PERSISTENCE_STORAGE.get_instance().valentines;
        &valentines[rng.gen_range(0, valentines.len())]
    }
}

fn get_first_name(name: &str) -> &str {
    let first_name: Vec<&str> = name.split(' ').collect();
    first_name[0]
}

fn get_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{}.png?v=1", emote_id)
}