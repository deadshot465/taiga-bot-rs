use serenity::{
    framework::standard::{
        macros::{
            command
        }, CommandResult
    },
    model::channel::Message,
    model::prelude::*,
    prelude::*
};
use crate::{CommandGroupCollection, InterfaceService};
use serenity::framework::standard::CommandGroup;
use serenity::utils::Color;
use std::sync::Arc;
use serenity::collector::ReactionAction;

const KOU_INTRO_PATH: &'static str = "persistence/kou_intro.txt";
const TAIGA_INTRO_PATH: &'static str = "persistence/taiga_intro.txt";

#[command]
#[description = "Start a step-by-step guide."]
#[usage = ""]
#[example = ""]
#[bucket = "information"]
pub async fn guide(context: &Context, msg: &Message) -> CommandResult {
    let data = context.data.read().await;
    let command_groups = data.get::<CommandGroupCollection>().unwrap().to_vec();
    let interface = data.get::<InterfaceService>().unwrap();
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(data);

    let mut text: String;
    let file: std::io::Result<String>;
    if is_kou {
        file = std::fs::read_to_string(KOU_INTRO_PATH);
    }
    else {
        file = std::fs::read_to_string(TAIGA_INTRO_PATH);
    }
    if let Ok(s) = file {
        text = s;
    }
    else {
        eprintln!("Error when reading from the text file: {:?}", file.err().unwrap());
        return Ok(());
    }
    text = text.replace("{user}", &msg.author.mention());
    let color_code = u32::from_str_radix(if is_kou {
        "a4d0da"
    } else {
        "e81615"
    }, 16).unwrap();
    build_embed(context, msg, &command_groups, Color::new(color_code), text.as_str(), is_kou)
        .await?;
    Ok(())
}

async fn build_embed(context: &Context, msg: &Message, command_groups: &Vec<&CommandGroup>, color: Color, text: &str, is_kou: bool) -> CommandResult {
    let http = &context.http;
    let author = msg.author
        .nick_in(http, msg.guild_id.clone().unwrap())
        .await
        .unwrap_or(msg.author.name.clone());
    let avatar_url = msg.author.avatar_url();
    let title = if is_kou {
        "Welcome to the Church of Minamoto Kou!"
    } else {
        "Welcome to Taiga's New Journal!"
    };
    let group_count = command_groups.len();
    let mut max_pages = group_count / 3;
    if group_count % 3 != 0 {
        max_pages += 1;
    }
    let mut current_page = 0;
    let prev_reaction = ReactionType::Unicode("⬅️".to_string());
    let next_reaction = ReactionType::Unicode("➡️".to_string());
    let end_reaction = ReactionType::Unicode("❌".to_string());
    let mut message: Message = msg.author.dm(http, |m| m.embed(|e| {
        e.author(|a| {
            a.name(&author);
            if let Some(u) = avatar_url.as_ref() {
                a.icon_url(u);
            }
            a
        });
        e.color(color);
        e.title(title);
        e.description(text);
        let start_index = 0 + 3 * current_page;
        let end_index = 3 + 3 * current_page;
        for i in start_index..end_index {
            let mut value = command_groups[i].options
                .description.unwrap_or_default().to_string();
            value += "\nList of commands: ";
            let command_names = command_groups[i].options.commands.iter()
                .map(|n| format!("`{}`", n.options.names[0]))
                .collect::<Vec<String>>();
            let concatenated: String = command_names.join(", ");
            value += concatenated.as_str();
            e.field(command_groups[i].name, &value, false);
        }
        e.footer(|f| f.text("Type `end` to end the guide!"));
        e
    })).await?;
    if current_page != 0 {
        message.react(http, prev_reaction.clone()).await?;
    }
    if current_page != max_pages - 1 {
        message.react(http, next_reaction.clone()).await?;
    }
    message.react(http, end_reaction.clone()).await?;
    loop {
        let mut delay = tokio::time::delay_for(tokio::time::Duration::from_secs(60 * 5));
        tokio::select! {
            _ = &mut delay => {
                message.channel_id.say(http, if is_kou {
                    "Thanks for taking a guide with me! I hope you can enjoy your stay! <a:KouFascinated:705279783340212265>"
                } else {
                    "Hope you like my guide! Make sure to say hello to other campers! <:chibitaiga:697893400891883531>"
                }).await?;
                break;
            }
            v = message.await_reaction(&context).author_id(msg.author.id) => {
                println!("Reaction received...");
                if let Some(r) = v {
                    let emoji: &ReactionType = &r.as_inner_ref().emoji;
                    match emoji.as_data().as_str() {
                        "➡️" => {
                            msg.author.dm(http, |m| m.content("Hello World!")).await?;
                            break;
                        },
                        _ => {

                        }
                    }
                }
            }
        }
    }
    Ok(())
}

