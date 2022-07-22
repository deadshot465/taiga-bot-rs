use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::config::configuration::KOU;
use crate::shared::structs::config::server_info::SERVER_INFOS;
use crate::shared::structs::fun::qotd::{QotdInfo, QOTD_INFOS};
use crate::shared::utility::extract_string_option;
use chrono::{TimeZone, Utc};
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::channel::ChannelType;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn qotd_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(qotd(ctx, command))
}

async fn qotd(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    if let Some(guild_id) = command.guild_id {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("Alright, I got your question! One moment...")
                })
            })
            .await?;

        let guild_creation_date = guild_id.created_at().naive_utc();
        let elapsed = Utc::now() - Utc.from_utc_datetime(&guild_creation_date);
        let elapsed_days = elapsed.num_days();
        let question = extract_string_option(&command, 0);

        let attachment = command
            .data
            .options
            .get(1)
            .and_then(|opt| opt.resolved.as_ref())
            .and_then(|value| {
                if let CommandDataOptionValue::Attachment(attachment) = value {
                    Some(attachment)
                } else {
                    None
                }
            });

        let is_kou = KOU.get().copied().unwrap_or(false);
        let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
        let current_user = ctx.http.get_current_user().await?;
        let avatar_url = current_user
            .avatar_url()
            .unwrap_or_else(|| current_user.default_avatar_url());

        for server_info in SERVER_INFOS.server_infos.iter() {
            for qotd_channel_id in server_info.qotd_channel_ids.iter() {
                if let Some(channel) = ctx.cache.guild_channel(*qotd_channel_id) {
                    let msg = channel
                        .send_message(&ctx.http, |msg| {
                            msg.embed(|embed| {
                                let result_embed = embed
                                    .title(format!("Day {}", elapsed_days))
                                    .color(color)
                                    .thumbnail(&avatar_url)
                                    .footer(|f| f.text("Answer qotd to earn 25 credits!"))
                                    .description(question);

                                if let Some(attachment) = attachment {
                                    result_embed.image(attachment.url.as_str());
                                }

                                result_embed
                            })
                        })
                        .await?;
                    let thread = channel
                        .create_public_thread(&ctx.http, msg.id, |thread| {
                            thread
                                .name(format!("Day {} QotD", elapsed_days))
                                .kind(ChannelType::PublicThread)
                                .auto_archive_duration(1440)
                        })
                        .await?;

                    let mut qotd_infos = QOTD_INFOS.write().await;
                    qotd_infos.qotd_infos.insert(
                        0,
                        QotdInfo {
                            thread_channel_id: thread.id.0,
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
