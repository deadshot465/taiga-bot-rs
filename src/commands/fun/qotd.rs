use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::config::configuration::KOU;
use crate::shared::structs::config::server_info::SERVER_INFOS;
use crate::shared::structs::fun::qotd::{QotdInfo, QOTD_INFOS};
use crate::shared::utility::extract_string_option;
use chrono::{TimeZone, Utc};
use serenity::all::{
    AutoArchiveDuration, ChannelId, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateThread,
};
use serenity::builder::CreateEmbed;
use serenity::model::application::CommandInteraction;
use serenity::model::channel::ChannelType;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn qotd_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(qotd(ctx, command))
}

async fn qotd(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    if let Some(guild_id) = command.guild_id {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Alright, I got your question! One moment..."),
                ),
            )
            .await?;

        let guild_creation_date = guild_id.created_at().naive_utc();
        let elapsed = Utc::now() - Utc.from_utc_datetime(&guild_creation_date);
        let elapsed_days = elapsed.num_days();
        let question = extract_string_option(&command, 0).to_string();

        let attachments = command
            .data
            .resolved
            .attachments
            .into_values()
            .collect::<Vec<_>>();

        let is_kou = KOU.get().copied().unwrap_or(false);
        let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
        let current_user = ctx.http.get_current_user().await?;
        let avatar_url = current_user
            .avatar_url()
            .unwrap_or_else(|| current_user.default_avatar_url());

        let guild_channels = ctx.cache.guild_channels(guild_id);

        for server_info in SERVER_INFOS.server_infos.iter() {
            for qotd_channel_id in server_info.qotd_channel_ids.iter() {
                if let Some(channel) = guild_channels
                    .as_ref()
                    .and_then(|channels| channels.get(&ChannelId::new(*qotd_channel_id)).cloned())
                {
                    let mut result_embed = CreateEmbed::new()
                        .title(format!("Day {}", elapsed_days))
                        .color(color)
                        .thumbnail(&avatar_url)
                        .footer(CreateEmbedFooter::new("Answer qotd to earn 25 credits!"))
                        .description(&question);

                    if !attachments.is_empty() {
                        result_embed = result_embed.image(attachments[0].url.clone())
                    }

                    let msg = channel
                        .send_message(&ctx.http, CreateMessage::new().embed(result_embed))
                        .await?;
                    let thread = channel
                        .create_thread_from_message(
                            &ctx.http,
                            msg.id,
                            CreateThread::new(format!("Day {} QotD", elapsed_days))
                                .kind(ChannelType::PublicThread)
                                .auto_archive_duration(AutoArchiveDuration::OneDay),
                        )
                        .await?;

                    let mut qotd_infos = QOTD_INFOS.write().await;
                    qotd_infos.qotd_infos.insert(
                        0,
                        QotdInfo {
                            thread_channel_id: thread.id.get(),
                            question: question.to_string(),
                            expiry: Utc::now() + chrono::Duration::days(1),
                            participated_members: vec![],
                        },
                    );
                    qotd_infos.write_qotd_infos()?;
                }
            }
        }
    }

    Ok(())
}
