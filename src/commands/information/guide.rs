use crate::event_handler::commands::{SlashCommandElements, AVAILABLE_COMMANDS};
use crate::shared::constants::{ASSET_DIRECTORY, KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::config::configuration::KOU;
use once_cell::sync::Lazy;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

static KOU_INTRO_TEXT: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string(String::from(ASSET_DIRECTORY) + "/txt/kou_intro.txt")
        .expect("Failed to read Kou's intro text from local disk.")
});

static TAIGA_INTRO_TEXT: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string(String::from(ASSET_DIRECTORY) + "/txt/taiga_intro.txt")
        .expect("Failed to read Taiga's intro text from local disk.")
});

pub fn guide_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(guide(ctx, command))
}

pub async fn inner_guide(ctx: &Context, guild: Guild, member: Member) -> anyhow::Result<()> {
    let is_kou = KOU.get().copied().unwrap_or(false);

    let text = if is_kou {
        KOU_INTRO_TEXT
            .replace("{user}", &member.user.name)
            .replace("{guildName}", &guild.name)
    } else {
        TAIGA_INTRO_TEXT
            .replace("{user}", &member.user.name)
            .replace("{guildName}", &guild.name)
    };

    let bot_user = ctx.http.get_current_user().await?;
    let bot_avatar_url = if let Some(avatar_url) = bot_user.avatar_url() {
        avatar_url
    } else {
        bot_user.default_avatar_url()
    };

    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let title = format!("Welcome to {}!", &guild.name);
    let thumbnail = guild.icon_url().unwrap_or_default();
    let embed = build_embed(
        &bot_user.name,
        &title,
        color,
        &bot_avatar_url,
        &text,
        &thumbnail,
    );

    Ok(())
}

fn build_component(ctx: &Context, member: &Member, embed: CreateEmbed) -> anyhow::Result<()> {
    let available_commands = AVAILABLE_COMMANDS
        .iter()
        .map(
            |(
                name,
                SlashCommandElements {
                    description, emoji, ..
                },
            )| (name.clone(), (description.clone(), emoji.clone())),
        )
        .collect::<HashMap<_, _>>();

    let sent_msg = member.user.direct_message(&ctx.http, |msg| {
        msg.set_embed(embed).components(|components| {
            components.create_action_row(|row| {
                row.create_select_menu(|menu| {
                    menu.min_values(1)
                        .max_values(1)
                        .custom_id("command")
                        .placeholder("Select a command!")
                        .options(|opts| {
                            for (name, (description, emoji)) in available_commands.into_iter() {
                                opts.create_option(|opt| {
                                    opt.description(&description)
                                        .emoji(ReactionType::Unicode(emoji))
                                        .value(description)
                                        .label(name)
                                });
                            }

                            opts
                        });

                    menu
                })
            })
        })
    });

    Ok(())
}

fn build_embed(
    author_name: &str,
    title: &str,
    color: Color,
    avatar_url: &str,
    description: &str,
    thumbnail: &str,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .author(|author| author.name(author_name).icon_url(avatar_url))
        .description(description)
        .title(title)
        .color(color)
        .thumbnail(thumbnail)
        .footer(|f| f.text("Type `end` to end the guide!"));
    embed
}

async fn guide(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    if let Some(guild_id) = command.guild_id {
        if let Some(member) = command.member {
            if let Some(guild) = ctx.cache.guild(guild_id).await {
                inner_guide(&ctx, guild, member).await?;
            }
        }
    }
    Ok(())
}
