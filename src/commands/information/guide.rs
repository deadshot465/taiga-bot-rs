use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use poise::CreateReply;
use serenity::all::{
    ButtonStyle, Color, ComponentInteractionDataKind, CreateActionRow, CreateButton,
    CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, Guild, Member, Message,
};
use serenity::builder::CreateEmbed;

use crate::shared::constants::{ASSET_DIRECTORY, KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::{Context, ContextError};

const KOU_GOODBYE: &str = "Thanks for taking a guide with me! I hope you can enjoy your stay! <a:KouFascinated:705279783340212265>";
const TAIGA_GOODBYE: &str = "Hope you like my guide! Make sure to say hello to other campers! <:chibitaiga:697893400891883531>";

static KOU_INTRO_TEXT: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string(String::from(ASSET_DIRECTORY) + "/txt/kou_intro.txt")
        .expect("Failed to read Kou's intro text from local disk.")
});

static TAIGA_INTRO_TEXT: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string(String::from(ASSET_DIRECTORY) + "/txt/taiga_intro.txt")
        .expect("Failed to read Taiga's intro text from local disk.")
});

/// Start a step-by-step guide.
#[poise::command(slash_command, category = "Information")]
pub async fn guide(ctx: Context<'_>) -> Result<(), ContextError> {
    ctx.send(CreateReply::default().content("Check your DM!"))
        .await?;

    if let Some(guild_id) = ctx.guild_id() {
        if let Some(member) = ctx.author_member().await {
            let guild = ctx
                .cache()
                .guild(guild_id)
                .map(|guild_ref| guild_ref.clone());

            let member = match member {
                Cow::Borrowed(m) => m.clone(),
                Cow::Owned(m) => m,
            };

            if let Some(guild) = guild {
                inner_guide(ctx, guild, member).await?;
            }
        }
    }
    Ok(())
}

pub async fn inner_guide(ctx: Context<'_>, guild: Guild, member: Member) -> anyhow::Result<()> {
    let is_kou = ctx.data().kou;

    let text = if is_kou {
        KOU_INTRO_TEXT
            .replace("{user}", &member.user.name)
            .replace("{guildName}", &guild.name)
    } else {
        TAIGA_INTRO_TEXT
            .replace("{user}", &member.user.name)
            .replace("{guildName}", &guild.name)
    };

    let bot_user = ctx.http().get_current_user().await?;
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

    let global_commands = ctx.http().get_global_commands().await?;

    let mut available_commands = global_commands
        .iter()
        .map(|command| (command.name.clone(), command.description.clone()))
        .collect::<Vec<_>>();
    available_commands.sort_unstable_by(|(name_1, _), (name_2, _)| name_1.cmp(name_2));

    let sent_msg = build_component(ctx, member.clone(), embed, &available_commands).await?;
    tour_loop(ctx, member, sent_msg, &available_commands, is_kou).await?;

    Ok(())
}

async fn build_component(
    ctx: Context<'_>,
    member: Member,
    embed: CreateEmbed,
    available_commands: &[(String, String)],
) -> anyhow::Result<Message> {
    let command_options = available_commands
        .iter()
        .map(|(name, _)| CreateSelectMenuOption::new(name.as_str(), name.as_str()))
        .collect::<Vec<_>>();

    let sent_msg = member
        .user
        .direct_message(
            ctx.http(),
            CreateMessage::new().embed(embed).components(vec![
                CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(
                        "command",
                        CreateSelectMenuKind::String {
                            options: command_options,
                        },
                    )
                    .min_values(1)
                    .max_values(1)
                    .placeholder("Select a command!"),
                ),
                CreateActionRow::Buttons(vec![CreateButton::new("end_tour")
                    .label("End Tour")
                    .style(ButtonStyle::Danger)]),
            ]),
        )
        .await?;

    Ok(sent_msg)
}

fn build_embed(
    author_name: &str,
    title: &str,
    color: Color,
    avatar_url: &str,
    description: &str,
    thumbnail: &str,
) -> CreateEmbed {
    CreateEmbed::new()
        .author(CreateEmbedAuthor::new(author_name).icon_url(avatar_url))
        .description(description)
        .title(title)
        .color(color)
        .thumbnail(thumbnail)
        .footer(CreateEmbedFooter::new(
            "Press `End Tour` button to end the guide!",
        ))
}

async fn tour_loop(
    ctx: Context<'_>,
    member: Member,
    sent_msg: Message,
    available_commands: &[(String, String)],
    is_kou: bool,
) -> anyhow::Result<()> {
    let available_commands = available_commands
        .iter()
        .map(|(name, description)| (name.clone(), description.clone()))
        .collect::<HashMap<_, _>>();

    'outer: loop {
        let delay = tokio::time::sleep(std::time::Duration::from_secs(60 * 5));
        tokio::pin!(delay);

        'inner: loop {
            let collector = sent_msg
                .await_component_interaction(ctx)
                .channel_id(sent_msg.channel_id)
                .author_id(member.user.id)
                .timeout(std::time::Duration::from_secs(60 * 5))
                .message_id(sent_msg.id);

            tokio::select! {
                _ = &mut delay => {
                    sent_msg.delete(ctx.http()).await?;
                    member.user
                        .direct_message(ctx.http(), CreateMessage::new()
                        .content(if is_kou {
                            KOU_GOODBYE
                        } else {
                            TAIGA_GOODBYE
                        })).await?;
                    break 'outer;
                }
                maybe_v = collector.next() => {
                    if let Some(ref interaction) = maybe_v {
                        match interaction.data.kind.clone() {
                            ComponentInteractionDataKind::Button => {
                                sent_msg.delete(ctx.http()).await?;
                                interaction
                                    .create_response(ctx.http(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                                    .content(if is_kou {
                                        KOU_GOODBYE
                                    } else {
                                        TAIGA_GOODBYE
                                    })))
                                    .await?;
                                break 'outer;
                            },
                            ComponentInteractionDataKind::StringSelect {
                                values
                            } => {
                                if let Some(value) = values.first() {
                                    if let Some(description) = available_commands.get(value.as_str()) {
                                        interaction
                                            .create_response(ctx.http(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                                            .content(format!("**{}**: {}", value.as_str(), *description))))
                                            .await?;
                                    }
                                }
                                break 'inner;
                            },
                            _ => break 'inner,
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
