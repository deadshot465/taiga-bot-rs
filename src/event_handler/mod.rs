use crate::event_handler::commands::AVAILABLE_COMMANDS;
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

        let config = CONFIGURATION.get().expect("Failed to get configuration.");
        if config.recreate_global_slash_commands {
            commands::build_global_slash_commands(&ctx)
                .await
                .expect("Failed to override global commands.");
        }

        commands::build_guild_slash_commands(&ctx)
            .await
            .expect("Failed to override guild commands.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Some(commands) = AVAILABLE_COMMANDS.get() {
                if let Some(cmd) = commands.get(&command.data.name) {
                    let execution_result = cmd(ctx, command).await;
                    if let Err(e) = execution_result {
                        log::error!("Failed to execute slash command. Error: {}", e);
                    }
                } else {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response.interaction_response_data(|data| {
                                data.content("Sorry, this command is not yet implemented!")
                            })
                        })
                        .await
                        .expect("Cannot find slash command.")
                }
            }
        }
    }
}
