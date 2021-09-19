use crate::event_handler::commands::{set_admin_commands_permission, AVAILABLE_COMMANDS};
use crate::shared::structs::config::configuration::CONFIGURATION;
use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::{async_trait, prelude::*};

pub mod commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is now online.", ready.user.name);

        let recreate_global_slash_commands = CONFIGURATION
            .get()
            .map(|c| c.recreate_global_slash_commands)
            .unwrap_or(false);

        if let Err(e) =
            commands::build_global_slash_commands(&ctx, recreate_global_slash_commands).await
        {
            log::error!("Failed to override global commands. Error: {}", e);
        }

        if let Err(e) = commands::build_guild_slash_commands(&ctx).await {
            log::error!("Failed to override guild commands. Error: {}", e);
        }

        if let Err(e) = set_admin_commands_permission(&ctx).await {
            log::error!("Failed to set admin commands permission. Error: {}", e);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Some(commands) = AVAILABLE_COMMANDS.get() {
                if let Some((cmd, _)) = commands.get(&command.data.name) {
                    let execution_result = cmd(ctx, command).await;
                    if let Err(e) = execution_result {
                        log::error!("Failed to execute slash command. Error: {}", e);
                    }
                } else {
                    let result = command
                        .create_interaction_response(&ctx.http, |response| {
                            response.interaction_response_data(|data| {
                                data.content("Sorry, this command is not yet implemented!")
                            })
                        })
                        .await;
                    if let Err(e) = result {
                        log::error!("Failed to execute slash command. Error: {}", e);
                    }
                }
            }
        }
    }
}
