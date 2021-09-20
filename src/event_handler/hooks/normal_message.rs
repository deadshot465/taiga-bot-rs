use crate::event_handler::responses::greet::greet;
use crate::shared::constants::{
    KOU_SERVER_CERTIFICATION_MESSAGE, KOU_SERVER_CERTIFIED_ROLE_ID, KOU_SERVER_ID,
    KOU_SERVER_RULE_CHANNEL_ID,
};
use serenity::framework::standard::macros::hook;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[hook]
pub async fn normal_message_hook(ctx: &Context, message: &Message) {
    if message.author.bot {
        return;
    }

    if let Some(ref guild_id) = message.guild_id {
        if guild_id.0 != KOU_SERVER_ID {
            return;
        }

        if let Err(e) = certify_user(&ctx, &message).await {
            log::error!("An error occurred when certifying the user: {}", e);
        }
    }
}

async fn certify_user(ctx: &Context, message: &Message) -> anyhow::Result<()> {
    if let Some(ref partial_member) = message.member {
        if message.channel_id.0 != KOU_SERVER_RULE_CHANNEL_ID {
            return Ok(());
        }

        if partial_member
            .roles
            .contains(&RoleId(KOU_SERVER_CERTIFIED_ROLE_ID))
        {
            return Ok(());
        }

        let guild = message
            .guild(&ctx.cache)
            .await
            .expect("Failed to retrieve guild from cache.");
        let mut member = message.member(&ctx.http).await?;

        if message.content.as_str() == KOU_SERVER_CERTIFICATION_MESSAGE {
            member
                .add_role(&ctx.http, RoleId(KOU_SERVER_CERTIFIED_ROLE_ID))
                .await?;

            greet(ctx, guild, member).await?;
            message.delete(&ctx.http).await?;
        } else {
            let reply_message = message
                .reply(&ctx.http, "Your answer is incorrect. Please try again.")
                .await?;
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            reply_message.delete(&ctx.http).await?;
            message.delete(&ctx.http).await?;
        }
    }

    Ok(())
}
