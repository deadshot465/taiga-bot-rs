use crate::{CommandGroupCollection, InterfaceService, PersistenceService};
use serenity::framework::standard::CommandGroup;
use serenity::utils::Color;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    model::prelude::*,
    prelude::*,
};

const GUIDE_ACTIONS: [&str; 3] = ["prev", "next", "end"];
const KOU_GOODBYE: &str = "Thanks for taking a guide with me! I hope you can enjoy your stay! <a:KouFascinated:705279783340212265>";
const TAIGA_GOODBYE: &str = "Hope you like my guide! Make sure to say hello to other campers! <:chibitaiga:697893400891883531>";

#[command]
#[description = "Start a step-by-step guide."]
#[usage = ""]
#[example = ""]
#[bucket = "information"]
async fn guide(context: &Context, msg: &Message) -> CommandResult {
    let data = context.data.read().await;
    let command_groups = data.get::<CommandGroupCollection>().unwrap().to_vec();
    let interface = data.get::<InterfaceService>().unwrap();
    let persistence = data.get::<PersistenceService>().unwrap();
    let interface_lock = interface.read().await;
    let persistence_lock = persistence.read().await;
    let is_kou = interface_lock.is_kou;
    let mut text = persistence_lock.guide_text.clone();
    drop(persistence_lock);
    drop(interface_lock);
    drop(data);

    let guild_name = msg
        .guild_id
        .as_ref()
        .unwrap()
        .name(&context.cache)
        .await
        .unwrap_or_default();
    text = text.replace("{user}", &msg.author.mention());
    text = text.replace("{guildName}", &guild_name);
    let color_code = u32::from_str_radix(if is_kou { "a4d0da" } else { "e81615" }, 16).unwrap();
    let member = msg.member(&context.http).await.unwrap();
    build_embed(
        context,
        &member,
        &command_groups,
        Color::new(color_code),
        text.as_str(),
        is_kou,
        guild_name.as_str(),
    )
    .await?;
    Ok(())
}

pub async fn build_embed(
    context: &Context,
    member: &Member,
    command_groups: &[&CommandGroup],
    color: Color,
    text: &str,
    is_kou: bool,
    guild_name: &str,
) -> CommandResult {
    let http = &context.http;
    let author = context
        .http
        .get_current_user()
        .await
        .expect("Failed to get current user.");
    let avatar_url = author.avatar_url();
    let title = format!("Welcome to {}!", guild_name);
    let group_count = command_groups.len();
    let mut max_pages = group_count / 3;
    if group_count % 3 != 0 {
        max_pages += 1;
    }
    let mut current_page = 0;
    /*
    let prev_reaction = ReactionType::Unicode("⬅️".to_string());
    let next_reaction = ReactionType::Unicode("➡️".to_string());
    let end_reaction = ReactionType::Unicode("❌".to_string());
    */
    let all_commands = command_groups
        .iter()
        .map(|c| c.options.commands)
        .collect::<Vec<_>>();
    let all_commands = all_commands.into_iter().flatten().collect::<Vec<_>>();
    let mut message: Message = member
        .user
        .dm(http, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(&author.name);
                    if let Some(u) = avatar_url.as_ref() {
                        a.icon_url(u);
                    }
                    a
                });
                e.color(color);
                e.title(&title);
                e.description(text);
                let start_index = 3 * current_page;
                let end_index = 3 + 3 * current_page;
                for command_group in command_groups.iter().take(end_index).skip(start_index) {
                    let mut value = command_group
                        .options
                        .description
                        .unwrap_or_default()
                        .to_string();
                    value += "\nList of commands: ";
                    let command_names = command_group
                        .options
                        .commands
                        .iter()
                        .map(|n| format!("`{}`", n.options.names[0]))
                        .collect::<Vec<String>>();
                    let concatenated: String = command_names.join(", ");
                    value += concatenated.as_str();
                    e.field(command_group.name, &value, false);
                }
                e.footer(|f| f.text("Type `end` to end the guide!"));
                e
            })
        })
        .await?;

    'outer: loop {
        let mut delay = tokio::time::delay_for(tokio::time::Duration::from_secs(60 * 5));
        message
            .edit(http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(&author.name);
                        if let Some(u) = avatar_url.as_ref() {
                            a.icon_url(u);
                        }
                        a
                    });
                    e.color(color);
                    e.title(&title);
                    e.description(text);
                    let start_index = 3 * current_page;
                    let end_index = 3 + 3 * current_page;
                    for command_group in command_groups.iter().take(end_index).skip(start_index) {
                        let mut value = command_group
                            .options
                            .description
                            .unwrap_or_default()
                            .to_string();
                        value += "\nList of commands: ";
                        let command_names = command_group
                            .options
                            .commands
                            .iter()
                            .map(|n| format!("`{}`", n.options.names[0]))
                            .collect::<Vec<String>>();
                        let concatenated: String = command_names.join(", ");
                        value += concatenated.as_str();
                        e.field(command_group.name, &value, false);
                    }
                    e.footer(|f| f.text("Type `end` to end the guide!"));
                    e
                })
            })
            .await?;
        /*
        if current_page != 0 {
            message.react(http, prev_reaction.clone()).await?;
        }
        if current_page != max_pages - 1 {
            message.react(http, next_reaction.clone()).await?;
        }
        message.react(http, end_reaction.clone()).await?;
        */
        'inner: loop {
            tokio::select! {
                _ = &mut delay => {
                    message.channel_id.say(http, if is_kou {
                        KOU_GOODBYE
                    } else {
                        TAIGA_GOODBYE
                    }).await?;
                    break 'outer;
                }
                maybe_v = member.user.await_reply(&context)
                .channel_id(message.channel_id.0)
                .author_id(member.user.id.0) => {
                    if let Some(m) = maybe_v {
                        let lower_case: String = m.content.to_lowercase();
                        if all_commands
                        .iter()
                        .map(|c| c.options.names[0])
                        .any(|x| x.contains(&lower_case.as_str())) {
                            let _msg = message.channel_id.send_message(http, |m| m.embed(|e| {
                                e.author(|a| {
                                    a.name(&author.name);
                                    if let Some(u) = avatar_url.as_ref() {
                                        a.icon_url(u);
                                    }
                                    a
                                });
                                e.title(&lower_case);
                                e.color(color);
                                let cmd = all_commands.iter()
                                    .find(|c| c.options.names.contains(&lower_case.as_str()))
                                    .unwrap();
                                e.description(cmd.options.desc.unwrap());
                                e
                            })).await?;
                            let _http = http.clone();
                            tokio::spawn(async move {
                                tokio::time::delay_for(tokio::time::Duration::from_secs(10)).await;
                                _msg.delete(_http).await.expect("Failed to delete the message.");
                            });
                        }
                        else if GUIDE_ACTIONS.contains(&lower_case.as_str()) {
                            match lower_case.as_str() {
                                "prev" => {
                                    if current_page == 0 {
                                        let _msg = message.channel_id.say(http, "You're already in the first page!")
                                            .await?;
                                        let _http = http.clone();
                                        tokio::spawn(async move {
                                            tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                                            _msg.delete(_http).await.expect("Failed to delete message.");
                                        });
                                    }
                                    else {
                                        current_page -= 1;
                                    }
                                },
                                "next" => {
                                    if current_page == max_pages - 1 {
                                        let _msg = message.channel_id.say(http, "You're already in the final page!")
                                            .await?;
                                        let _http = http.clone();
                                        tokio::spawn(async move {
                                            tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                                            _msg.delete(_http).await.expect("Failed to delete message.");
                                        });
                                    }
                                    else {
                                        current_page += 1;
                                    }
                                },
                                "end" => {
                                    message.channel_id.say(http, if is_kou {
                                        KOU_GOODBYE
                                    } else {
                                        TAIGA_GOODBYE
                                    }).await?;
                                    break 'outer;
                                },
                                _ => ()
                            }
                        }
                        break 'inner;
                    }
                }
            }
        }
    }
    Ok(())
}
