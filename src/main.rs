use env_logger::Builder;
use log::LevelFilter;

mod commands;
mod event_handler;
mod shared;
use crate::event_handler::Handler;
use crate::shared::structs::config::channel_control::CHANNEL_CONTROL;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity::Client;
use shared::structs::config::*;
use shared::structs::record::*;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    random_response::initialize()?;
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
        log::error!("Discord token cannot be empty.");
        return Ok(());
    }

    let mut client = {
        let config = configuration::CONFIGURATION
            .get()
            .expect("Configuration is not initialized.");
        let log_level = match config.log_level.as_str() {
            "DEBUG" => LevelFilter::Debug,
            "INFO" => LevelFilter::Info,
            "WARN" => LevelFilter::Warn,
            "ERROR" => LevelFilter::Error,
            "TRACE" => LevelFilter::Trace,
            "OFF" => LevelFilter::Off,
            _ => LevelFilter::Debug,
        };

        Builder::new()
            .filter(None, log_level)
            .default_format()
            .init();

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

        let http = Http::new_with_token(token);
        let app_info = http.get_current_application_info().await?;
        let mut owners = HashSet::new();
        owners.insert(app_info.owner.id);

        Client::builder(token)
            .event_handler(Handler)
            .intents(GatewayIntents::all())
            .application_id(*application_id)
            .framework(StandardFramework::new().configure(|c| {
                c.prefix(prefix)
                    .allow_dm(false)
                    .allowed_channels(enabled_channels)
                    .owners(owners)
            }))
            .await?
    };

    if let Err(e) = client.start().await {
        log::error!("Error when starting the bot: {}", e.to_string());
    }

    Ok(())
}
