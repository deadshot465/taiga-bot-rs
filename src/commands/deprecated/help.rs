use crate::InterfaceService;
use serenity::all::Color;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands, Args, Command, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::prelude::Context;
use std::collections::{HashMap, HashSet};

#[help]
#[individual_command_tip = "Here is a list of all commands and their descriptions."]
#[command_not_found_text = "Shit, `{}` failed."]
#[embed_success_colour("Blitz_Blue")]
#[max_levenshtein_distance(3)]
pub async fn custom_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut group_names = groups.iter().map(|g| g.name).collect::<Vec<&'static str>>();
    group_names.sort_unstable();
    let mut group_commands = HashMap::new();
    let _ = groups
        .iter()
        .map(|g| group_commands.insert(g.name, g.options.commands))
        .collect::<Vec<Option<&[&Command]>>>();

    let member = msg.member(&context.http).await.unwrap();
    let color: u32;
    let data = context.data.read().await;
    let interface = data.get::<InterfaceService>().unwrap();
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(data);
    if is_kou {
        color = u32::from_str_radix("a4d0da", 16).unwrap();
    } else {
        color = u32::from_str_radix("e81615", 16).unwrap();
    }

    if args.is_empty() {
        msg.channel_id
            .send_message(&context.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        let name: String;
                        if let Some(nick) = member.nick.as_ref() {
                            name = nick.clone();
                        } else {
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
                        let mut cmds = group_commands[*group]
                            .iter()
                            .map(|c| c.options.names[0])
                            .collect::<Vec<&'static str>>();
                        cmds.sort_unstable();
                        let cmds_strings: String =
                            cmds.iter().map(|c| format!("`{}` ", c)).collect();
                        e.field(
                            *group,
                            &cmds_strings,
                            !(column_no != 0 && column_no % 3 == 0),
                        );
                    }
                    e.color(Color::from(color))
                })
            })
            .await?;
        Ok(())
    } else {
        let mut _args = args.clone();
        let mut arg = _args.single::<String>().unwrap();
        if _args.remaining() > 0 {
            help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
            return Ok(());
        }
        arg = arg.to_lowercase();
        let mut chars = arg.chars().collect::<Vec<_>>();
        chars[0] = chars[0].to_uppercase().next().unwrap();
        let arg = chars.into_iter().collect::<String>();
        if group_names.contains(&arg.as_str()) {
            let commands = *group_commands
                .get(&arg.as_str())
                .expect("Failed to get group of commands.");
            let group = *groups.iter().find(|g| g.name == arg.as_str()).unwrap();
            msg.channel_id
                .send_message(&context.http, |m| {
                    m.embed(|e| {
                        e.author(|a| {
                            a.name(member.nick.as_ref().unwrap_or(&member.user.name));
                            if let Some(u) = member.user.avatar_url() {
                                a.icon_url(&u);
                            }
                            a
                        });
                        e.color(Color::from(color));
                        e.title(group.name);
                        let mut description = String::new();
                        if let Some(d) = group.options.description {
                            description += d;
                        }
                        description += "\n**Prefixes:** ";
                        let prefixes = group
                            .options
                            .prefixes
                            .iter()
                            .map(|p| format!("`{}`", *p))
                            .collect::<Vec<_>>();
                        description += &prefixes.join(", ");
                        e.description(&description);
                        for cmd in commands {
                            e.field(
                                cmd.options.names[0],
                                cmd.options.desc.unwrap_or_default(),
                                false,
                            );
                        }
                        e
                    })
                })
                .await?;
            Ok(())
        } else {
            help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
            Ok(())
        }
    }
}
