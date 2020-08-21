use serenity::framework::standard::macros::{
    help
};
use serenity::framework::standard::{CommandResult, Args, help_commands, HelpOptions, CommandGroup, Command};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::collections::{HashSet, HashMap};
use serenity::model::id::UserId;
use crate::InterfaceService;
use serenity::utils::Color;

#[help]
#[individual_command_tip = "Here is a list of all commands and their descriptions."]
#[command_not_found_text = "Shit, `{}` failed."]
#[embed_success_colour("Blitz_Blue")]
#[max_levenshtein_distance(3)]
pub async fn custom_help(context: &Context, msg: &Message, args: Args,
                         help_options: &'static HelpOptions, groups: &[&'static CommandGroup],
                         owners: HashSet<UserId>) -> CommandResult {

    let mut group_names = groups.iter()
        .map(|g| g.name)
        .collect::<Vec<&'static str>>();
    group_names.sort();
    let mut group_commands = HashMap::new();
    let _ = groups.iter()
        .map(|g| group_commands.insert(g.name, g.options.commands))
        .collect::<Vec<Option<&[&Command]>>>();

    let member = msg.member(&context.cache).await.unwrap();
    let color: u32;
    let data = context.data.read().await;
    let interface = data.get::<InterfaceService>().unwrap();
    let interface_lock = interface.lock().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(data);
    if is_kou {
        color = u32::from_str_radix("a4d0da", 16).unwrap();
    }
    else {
        color = u32::from_str_radix("e81615", 16).unwrap();
    }

    if args.is_empty() {
        msg.channel_id.send_message(&context.http, |m| m
            .embed(|e| {
                e
                    .author(|a| {
                        let name: String;
                        if let Some(nick) = member.nick.as_ref() {
                            name = nick.clone();
                        }
                        else {
                            name = member.user.name.clone();
                        }
                        if let Some(url) = msg.author.avatar_url() {
                            a.icon_url(url.as_str());
                        }
                        a.name(&name)
                    })
                    .description("Here is a list of all commands and their descriptions.");

                let column_no = 0;
                for group in group_names.iter() {
                    let mut cmds = group_commands[*group].iter()
                        .map(|c| c.options.names[0])
                        .collect::<Vec<&'static str>>();
                    cmds.sort();
                    let cmds_strings: String = cmds.iter()
                        .map(|c| format!("`{}` ", c))
                        .collect();
                    e.field(*group, &cmds_strings, if column_no != 0 && column_no % 3 == 0 {
                        false
                    } else {
                        true
                    });
                }
                e.color(Color::from(color))
            })).await?;
        Ok(())
    }
    else {
        help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
        Ok(())
    }
}