use chrono::{TimeZone, Utc};
use poise::CreateReply;
use serenity::all::{
    Attachment, AutoArchiveDuration, ChannelId, CreateEmbedFooter, CreateMessage, CreateThread,
};
use serenity::builder::CreateEmbed;
use serenity::model::channel::ChannelType;

use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::fun::qotd::QotdInfo;
use crate::shared::structs::{Context, ContextError};

/// Ask a question of the day and earn 25 credits.
#[poise::command(slash_command, category = "Fun")]
pub async fn qotd(
    ctx: Context<'_>,
    #[description = "The question of the day to ask."]
    #[min_length = 10]
    question: String,
    #[description = "The attachment to add to the question of the day."] attachment: Option<
        Attachment,
    >,
) -> Result<(), ContextError> {
    let guild_id = ctx.guild_id();

    if guild_id.is_none() {
        ctx.send(
            CreateReply::default().content("Question of the day can only be posted in a guild!"),
        )
        .await?;
        return Ok(());
    }

    let guild_id = guild_id.unwrap_or_default();

    ctx.send(CreateReply::default().content("Alright, I got your question! One moment..."))
        .await?;

    let guild_creation_date = guild_id.created_at().naive_utc();
    let elapsed = Utc::now() - Utc.from_utc_datetime(&guild_creation_date);
    let elapsed_days = elapsed.num_days();
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };

    let current_user = ctx.http().get_current_user().await?;
    let avatar_url = current_user
        .avatar_url()
        .unwrap_or_else(|| current_user.default_avatar_url());

    let guild_channels = ctx.cache().guild(guild_id).map(|g| g.channels.clone());

    for server_info in ctx.data().server_infos.server_infos.iter() {
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

                if let Some(ref image) = attachment {
                    result_embed = result_embed.image(&image.url)
                }

                let msg = channel
                    .send_message(ctx.http(), CreateMessage::new().embed(result_embed))
                    .await?;
                let thread = channel
                    .create_thread_from_message(
                        ctx.http(),
                        msg.id,
                        CreateThread::new(format!("Day {} QotD", elapsed_days))
                            .kind(ChannelType::PublicThread)
                            .auto_archive_duration(AutoArchiveDuration::OneDay),
                    )
                    .await?;

                let qotd_infos = ctx.data().qotd_infos.clone();
                let mut qotd_infos_write_lock = qotd_infos.write().await;
                qotd_infos_write_lock.qotd_infos.insert(
                    0,
                    QotdInfo {
                        thread_channel_id: thread.id.get(),
                        question: question.to_string(),
                        expiry: Utc::now() + chrono::Duration::days(1),
                        participated_members: vec![],
                    },
                );
                qotd_infos_write_lock.write_qotd_infos()?;
            }
        }
    }

    Ok(())
}
