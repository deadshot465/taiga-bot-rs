use crate::shared::constants::{KOU_SERVER_SMOTE_ROLE_ID, TAIGA_SERVER_SMOTE_ROLE_ID};
use crate::shared::structs::smite::{SmoteUser, SMITE_GIF_LINKS, SMOTE_USERS};
use chrono::Utc;
use rand::prelude::*;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
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
            if let CommandDataOptionValue::User(user, _) = value {
                Some(user)
            } else {
                None
            }
        });

    let smote_member = if let Some(smote_user) = smote_user {
        if let Some(guild_id) = command.guild_id {
            ctx.http.get_member(guild_id.0, smote_user.id.0).await.ok()
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

                {
                    let mut smote_users_write_lock = SMOTE_USERS.write().await;
                    smote_users_write_lock.smote_users.push(SmoteUser {
                        user_id: member.user.id.0,
                        due_time: Utc::now() + chrono::Duration::days(1),
                        guild_id: command.guild_id.unwrap_or_default().0,
                    });
                    smote_users_write_lock.write_smote_user_list()?;
                }

                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    let ctx = ctx_clone;
                    tokio::time::sleep(std::time::Duration::from_secs(86400)).await;

                    match member.remove_role(&ctx.http, RoleId(role_id)).await {
                        Ok(_) => {
                            let mut smote_users_write_lock = SMOTE_USERS.write().await;
                            let filtered_user_list = smote_users_write_lock
                                .smote_users
                                .clone()
                                .into_iter()
                                .filter(|u| u.user_id != member.user.id.0)
                                .collect::<Vec<_>>();
                            smote_users_write_lock.smote_users = filtered_user_list;
                            if let Err(e) = smote_users_write_lock.write_smote_user_list() {
                                tracing::error!(
                                    "Error when writing smote user list to local disk: {}",
                                    e
                                );
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error when remove smote role from user: {}", e);
                        }
                    }
                });

                break;
            }
        }
    }

    Ok(())
}
