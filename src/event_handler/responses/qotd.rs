use crate::shared::services::credit_service::add_user_credit;
use crate::shared::structs::ContextData;
use crate::shared::utility::get_author_name;
use chrono::Utc;
use serenity::model::prelude::Message;
use serenity::prelude::*;

const REWARD: i32 = 25;

pub async fn handle_qotd(
    ctx: &Context,
    new_message: &Message,
    data: &ContextData,
) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }

    let qotd_infos = data.qotd_infos.clone();

    let qotd_exists = {
        qotd_infos
            .read()
            .await
            .qotd_infos
            .iter()
            .find(|info| info.thread_channel_id == new_message.channel_id.get())
            .cloned()
    };

    let member = new_message.member(&ctx.http).await.ok();

    let author_name = get_author_name(&new_message.author, &member);

    if let Some(qotd_info) = qotd_exists {
        if Utc::now() > qotd_info.expiry
            || qotd_info
                .participated_members
                .contains(&new_message.author.id.get())
        {
            Ok(())
        } else {
            {
                let mut qotd_infos = qotd_infos.write().await;
                let qotd_info = qotd_infos
                    .qotd_infos
                    .iter_mut()
                    .find(|info| info.thread_channel_id == new_message.channel_id.get());
                if let Some(info) = qotd_info {
                    info.participated_members.push(new_message.author.id.get());
                }
                qotd_infos.write_qotd_infos()?;
            }
            add_user_credit(new_message.author.id.get(), &author_name, REWARD, data).await?;
            new_message
                .reply(
                    &ctx.http,
                    "Thanks for your participation! You've earned 25 credits!",
                )
                .await?;
            Ok(())
        }
    } else {
        Ok(())
    }
}
