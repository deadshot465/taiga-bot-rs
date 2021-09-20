use crate::shared::constants::{KOU_SERVER_SMOTE_ROLE_ID, TAIGA_SERVER_SMOTE_ROLE_ID};
use crate::shared::structs::smite::SMITE_GIF_LINKS;
use rand::prelude::*;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn smite_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(smite(ctx, command))
}

async fn smite(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let smote_user = command
        .data
        .options
        .get(0)
        .and_then(|opt| opt.resolved.as_ref())
        .and_then(|value| {
            if let ApplicationCommandInteractionDataOptionValue::User(user, _) = value {
                Some(user)
            } else {
                None
            }
        });

    let smote_member = if let Some(smote_user) = smote_user {
        if let Some(guild_id) = command.guild_id {
            ctx.cache.member(guild_id, smote_user.id).await
        } else {
            None
        }
    } else {
        None
    };

    let smote_role_ids = vec![TAIGA_SERVER_SMOTE_ROLE_ID, KOU_SERVER_SMOTE_ROLE_ID];
    if let Some(mut member) = smote_member {
        for role_id in smote_role_ids.into_iter() {
            if member.add_role(&ctx.http, RoleId(role_id)).await.is_ok() {
                let gif_link = {
                    let mut rng = rand::thread_rng();
                    SMITE_GIF_LINKS
                        .choose(&mut rng)
                        .map(|s| s.as_str())
                        .unwrap_or_default()
                };
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response.interaction_response_data(|data| data.content(gif_link))
                    })
                    .await?;
                break;
            }
        }
    }

    Ok(())
}
