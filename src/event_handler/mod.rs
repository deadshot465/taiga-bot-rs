use crate::event_handler::commands::{
    set_commands_permission, SlashCommandElements, AVAILABLE_COMMANDS, SKIP_CHANNEL_CHECK_COMMANDS,
};
use crate::event_handler::presences::set_initial_presence;
use crate::event_handler::responses::emote::handle_emote;
use crate::event_handler::responses::greet::greet;
use crate::event_handler::responses::mention::handle_mention_self;
use crate::event_handler::responses::reaction::handle_reactions;
use crate::event_handler::responses::response::handle_responses;
use crate::shared::constants::KOU_SERVER_ID;
use crate::shared::structs::config::channel_control::CHANNEL_CONTROL;
use crate::shared::structs::config::configuration::CONFIGURATION;
use crate::shared::structs::smite::schedule_unsmite;
use rand::Rng;
use serenity::model::prelude::*;
use serenity::{async_trait, prelude::*};

pub mod commands;
pub mod hooks;
pub mod presences;
pub mod responses;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        if guild_id.0 == KOU_SERVER_ID {
            return;
        }

        if let Some(guild) = ctx.cache.guild(guild_id).await {
            if let Err(e) = greet(&ctx, guild, new_member).await {
                log::error!("Error when greeting a new member: {}", e);
            }
        }
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let is_channel_ignored = {
            CHANNEL_CONTROL
                .get()
                .expect("Failed to get channel control.")
                .read()
                .await
                .ignored_channels
                .iter()
                .any(|channel_id| *channel_id == new_message.channel_id.0)
        };

        if is_channel_ignored {
            return;
        }

        if let Err(e) = handle_mention_self(&ctx, &new_message).await {
            log::error!("Failed to reply to self mention: {}", e);
        }

        if let Err(e) = handle_reactions(&ctx, &new_message).await {
            log::error!("Failed to react to the message: {}", e);
        }

        if let Err(e) = handle_responses(&ctx, &new_message).await {
            log::error!("Failed to reply to the message: {}", e);
        }

        if let Err(e) = handle_emote(&ctx, &new_message).await {
            log::error!("Failed to send emote: {}", e);
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

        if let Err(e) = set_commands_permission(&ctx).await {
            log::error!("Failed to set admin commands permission. Error: {}", e);
        }

        set_initial_presence(&ctx).await;
        schedule_unsmite(&ctx).await;

        log::info!("{} is now online.", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let channel_enabled = {
                CHANNEL_CONTROL
                    .get()
                    .expect("Failed to get channel control.")
                    .read()
                    .await
                    .enabled_channels
                    .iter()
                    .any(|channel_id| *channel_id == command.channel_id.0)
            };

            if !channel_enabled
                && !SKIP_CHANNEL_CHECK_COMMANDS.contains(&command.data.name.as_str())
            {
                if let Err(e) = command.create_interaction_response(&ctx.http, |responses| responses
                    .interaction_response_data(|data| data
                        .content("This channel has to be enabled first before you can use command here!")))
                    .await {
                    log::error!("Error when responding to slash command: {}", e);
                }

                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                if let Err(e) = command
                    .delete_original_interaction_response(&ctx.http)
                    .await
                {
                    log::error!(
                        "Error when deleting original response to slash command: {}",
                        e
                    );
                }
                return;
            }

            if let Some(SlashCommandElements { handler, .. }) =
                AVAILABLE_COMMANDS.get(&command.data.name)
            {
                let execution_result = handler(ctx, command).await;
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

pub fn hit_or_miss(probability: i32) -> bool {
    rand::thread_rng().gen_range(0..100) < probability
}
