use std::collections::HashMap;

use poise::{CreateReply, execute_modal_on_component_interaction, serenity_prelude as serenity};
use serenity::all::{
    ChannelId, ComponentInteractionDataKind, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption,
};
use serenity::builder::CreateMessage;

use crate::shared::structs::{Context, ContextError};

#[derive(Debug, Clone, Default, poise::Modal)]
#[name = "Answer Qotd Anonymously"]
struct AnswerAnonModal {
    #[placeholder = "Your answer to the qotd."]
    #[min_length = 1]
    #[paragraph]
    pub answer: String,
}

/// Answer question of the day anonymously. Note you won't get any credits when answering anonymously.
#[poise::command(slash_command, dm_only)]
pub async fn answer_anon(ctx: Context<'_>) -> Result<(), ContextError> {
    ctx.defer().await?;

    let author = ctx.author();
    let guild_info_map = ctx
        .http()
        .get_guilds(None, None)
        .await?
        .into_iter()
        .map(|guild_info| (guild_info.id, guild_info))
        .collect::<HashMap<_, _>>();

    let mut available_guilds = vec![];
    for (guild_id, _guild_info) in guild_info_map.iter() {
        let member = ctx
            .http()
            .get_guild_members(*guild_id, None, None)
            .await?
            .into_iter()
            .map(|m| m.user.id)
            .find(|id| id.get() == author.id.get());
        if member.is_some() {
            available_guilds.push(*guild_id);
        }
    }

    let mut threads_by_guild_id = HashMap::new();
    for guild_id in available_guilds.into_iter() {
        let active_threads = ctx.http().get_guild_active_threads(guild_id).await?;
        threads_by_guild_id.insert(guild_id, active_threads.threads);
    }

    let server_info_qotd_channel_map = ctx
        .data()
        .server_infos
        .server_infos
        .iter()
        .map(|info| (info.server_id, info.qotd_channel_ids.clone()))
        .collect::<HashMap<_, _>>();

    let threads_by_guild_id = threads_by_guild_id
        .into_iter()
        .map(|(guild_id, threads)| {
            let valid_threads = threads
                .into_iter()
                .filter(|t| {
                    let parent_id = t.parent_id.map(|id| id.get()).unwrap_or_default();
                    server_info_qotd_channel_map[&guild_id.get()].contains(&parent_id)
                })
                .collect::<Vec<_>>();
            (guild_id, valid_threads)
        })
        .filter(|(_guild_id, threads)| !threads.is_empty())
        .collect::<Vec<_>>();

    let options = threads_by_guild_id
        .into_iter()
        .flat_map(|(guild_id, channel)| {
            channel
                .iter()
                .map(|c| {
                    let guild_name = guild_info_map[&guild_id].name.as_str();
                    CreateSelectMenuOption::new(
                        format!("{} in {}", &c.name, guild_name),
                        format!("{}:{}", guild_id.get(), c.id.get()),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let custom_id = "qotd".to_string();
    let reply = {
        let components = vec![serenity::CreateActionRow::SelectMenu(
            CreateSelectMenu::new(&custom_id, CreateSelectMenuKind::String { options })
                .placeholder("Select a qotd!")
                .min_values(1)
                .max_values(1),
        )];

        CreateReply::default()
            .content("Please select the qotd you want to answer to in 5 minutes!")
            .components(components)
    };

    let reply_handle = ctx.send(reply).await?;

    if let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(author.id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(300))
        .filter(move |mci| mci.data.custom_id == custom_id)
        .await
    {
        if let ComponentInteractionDataKind::StringSelect { values } = mci.clone().data.kind {
            if let Context::Application(context) = ctx {
                let index = values[0].find(':').unwrap_or_default();
                let thread_id = &values[0][(index + 1)..];
                let thread_id = ChannelId::new(thread_id.parse::<u64>().unwrap_or_default());

                if let Some(modal_data) = execute_modal_on_component_interaction(
                    context,
                    mci,
                    None::<AnswerAnonModal>,
                    None,
                )
                .await?
                {
                    let channel = ctx.http().get_channel(thread_id).await?;
                    if let Some(guild_channel) = channel.guild() {
                        guild_channel
                            .send_message(
                                ctx.http(),
                                CreateMessage::new()
                                    .content(format!("Anonymous: {}", modal_data.answer)),
                            )
                            .await?;

                        reply_handle.delete(ctx).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
