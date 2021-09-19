use crate::event_handler::commands::{set_admin_commands_permission, AVAILABLE_COMMANDS};
use crate::event_handler::presences::set_initial_presence;
use crate::event_handler::responses::mention::handle_mention_self;
use crate::event_handler::responses::reaction::handle_reactions;
use crate::shared::structs::config::configuration::CONFIGURATION;
use rand::Rng;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::{async_trait, prelude::*};

pub mod commands;
pub mod presences;
pub mod responses;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if let Err(e) = handle_mention_self(&ctx, &new_message).await {
            log::error!("Failed to reply to self mention: {}", e);
        }

        if let Err(e) = handle_reactions(&ctx, &new_message).await {
            log::error!("Failed to react to the message: {}", e);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
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

        set_initial_presence(&ctx).await;

        log::info!("{} is now online.", ready.user.name);
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

pub fn hit_or_miss(probability: i32) -> bool {
    rand::thread_rng().gen_range(0..100) < probability
}
