use crate::event_handler::responses::greet::greet;
use crate::shared::constants::{
    KOU_SERVER_CERTIFICATION_MESSAGE, KOU_SERVER_CERTIFIED_ROLE_ID, KOU_SERVER_ID,
    KOU_SERVER_RULE_CHANNEL_ID,
};
use crate::shared::structs::ContextData;
use serenity::all::{Context, Message, RoleId};

pub async fn handle_certify(ctx: &Context, message: &Message, data: &ContextData) {
    if message.author.bot {
        return;
    }

    if let Some(ref guild_id) = message.guild_id {
        if guild_id.get() != KOU_SERVER_ID {
            return;
        }

        if let Err(e) = certify_user(ctx, message, data).await {
            tracing::error!("An error occurred when certifying the user: {}", e);
        }
    }
}

async fn certify_user(ctx: &Context, message: &Message, data: &ContextData) -> anyhow::Result<()> {
    if let Some(ref partial_member) = message.member {
        if message.channel_id.get() != KOU_SERVER_RULE_CHANNEL_ID {
            return Ok(());
        }

        if partial_member
            .roles
            .contains(&RoleId::new(KOU_SERVER_CERTIFIED_ROLE_ID))
        {
            return Ok(());
        }

        let guild = message
            .guild(&ctx.cache)
            .expect("Failed to retrieve guild from cache.")
            .clone();
        let member = message.member(&ctx.http).await?;

        if message.content.as_str() == KOU_SERVER_CERTIFICATION_MESSAGE {
            member
                .add_role(&ctx.http, RoleId::new(KOU_SERVER_CERTIFIED_ROLE_ID))
                .await?;

            greet(ctx, guild, &member, data).await?;
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
