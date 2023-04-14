mod commands;
mod event_handler;
mod shared;
use crate::commands::utility::eval::EVAL_COMMAND;
use crate::event_handler::hooks::normal_message::normal_message_hook;
use crate::event_handler::Handler;
use crate::shared::structs::config::channel_control::CHANNEL_CONTROL;
use serenity::framework::{standard::macros::group, StandardFramework};
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use shared::structs::config::*;
use shared::structs::record::*;
use std::collections::HashSet;
use tracing::Level;

#[group]
#[description = "Utility functions that basically serve as tools."]
#[only_in("guilds")]
#[commands(eval)]
struct Utility;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    configuration::initialize()?;
    channel_control::initialize()?;
    user_record::initialize()?;
    event_handler::commands::initialize();

    if configuration::CONFIGURATION
        .get()
        .map(|c| c.token.as_str())
        .unwrap_or_default()
        .is_empty()
    {
        return Err(anyhow::anyhow!("Discord token cannot be empty."));
    }

    let mut client = {
        let config = configuration::CONFIGURATION
            .get()
            .expect("Configuration is not initialized.");
        let log_level = match config.log_level.as_str() {
            "DEBUG" => Level::DEBUG,
            "INFO" => Level::INFO,
            "WARN" => Level::WARN,
            "ERROR" => Level::ERROR,
            "TRACE" => Level::TRACE,
            _ => Level::DEBUG,
        };

        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(log_level)
            .pretty()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            return Err(anyhow::anyhow!("Initializing tracing failed: {}", e));
        }

        let args = std::env::args()
            .map(|arg| arg.to_lowercase())
            .collect::<Vec<_>>();
        configuration::KOU.get_or_init(|| args.contains(&"kou".to_string()));
        let token = &config.token;
        let prefix = &config.prefix;
        let application_id = &config.application_id;
        let enabled_channels = {
            let channel_control = CHANNEL_CONTROL
                .get()
                .expect("Failed to get channel control.");
            let channel_control_read_lock = channel_control.read().await;
            channel_control_read_lock
                .enabled_channels
                .iter()
                .map(|n| ChannelId::from(*n))
                .collect::<HashSet<_>>()
        };

        let http = Http::new(token);
        let app_info = http.get_current_application_info().await?;
        let mut owners = HashSet::new();
        owners.insert(app_info.owner.id);

        Client::builder(token, GatewayIntents::all())
            .event_handler(Handler)
            .application_id(*application_id)
            .framework(
                StandardFramework::new()
                    .configure(|c| {
                        c.prefix(prefix)
                            .allow_dm(false)
                            .allowed_channels(enabled_channels)
                            .owners(owners)
                    })
                    .normal_message(normal_message_hook)
                    .group(&UTILITY_GROUP),
            )
            .await?
    };

    if let Err(e) = client.start().await {
        tracing::error!("Error when starting the bot: {}", e.to_string());
    }

    Ok(())
}
