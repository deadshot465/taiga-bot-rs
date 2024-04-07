use crate::shared::constants::{KOU_SERVER_SMOTE_ROLE_ID, TAIGA_SERVER_SMOTE_ROLE_ID};
use crate::shared::structs::smite::SmoteUser;
use crate::shared::structs::{Context, ContextError};
use chrono::Utc;
use poise::CreateReply;
use rand::prelude::*;
use serenity::all::{RoleId, User};

/// Smite bad behaving members.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn smite(
    ctx: Context<'_>,
    #[description = "Bad behaving member to smite."] member: User,
) -> Result<(), ContextError> {
    let smote_member = if let Some(guild_id) = ctx.guild_id() {
        ctx.http().get_member(guild_id, member.id).await.ok()
    } else {
        None
    };

    let smote_role_ids = vec![TAIGA_SERVER_SMOTE_ROLE_ID, KOU_SERVER_SMOTE_ROLE_ID];
    if let Some(member) = smote_member {
        for role_id in smote_role_ids.into_iter() {
            if member
                .add_role(ctx.http(), RoleId::new(role_id))
                .await
                .is_ok()
            {
                let gif_link = {
                    let mut rng = thread_rng();
                    ctx.data()
                        .smite
                        .smite_gif_links
                        .choose(&mut rng)
                        .map(|s| s.as_str())
                        .unwrap_or_default()
                };
                ctx.send(CreateReply::default().content(gif_link)).await?;

                let smote_user_list = ctx.data().smite.smote_user_list.clone();
                {
                    let mut smote_users_write_lock = smote_user_list.write().await;
                    smote_users_write_lock.smote_users.push(SmoteUser {
                        user_id: member.user.id.get(),
                        due_time: Utc::now() + chrono::Duration::days(1),
                        guild_id: ctx.guild_id().map(|id| id.get()).unwrap_or_default(),
                    });
                    smote_users_write_lock.write_smote_user_list()?;
                }

                let context = ctx.serenity_context().clone();
                tokio::spawn(async move {
                    let context = context;
                    let smote_user_list = smote_user_list;
                    tokio::time::sleep(std::time::Duration::from_secs(86400)).await;

                    match member.remove_role(context.http, RoleId::new(role_id)).await {
                        Ok(_) => {
                            let mut smote_users_write_lock = smote_user_list.write().await;
                            let filtered_user_list = smote_users_write_lock
                                .smote_users
                                .clone()
                                .into_iter()
                                .filter(|u| u.user_id != member.user.id.get())
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
