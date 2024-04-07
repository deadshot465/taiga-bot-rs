mod commands;
mod event_handler;
mod shared;

use crate::commands::information::about::about;
use crate::shared::structs::{ContextData, ContextError};

use crate::commands::admin::admin;
use crate::commands::fun::dialog::dialog;
use crate::commands::fun::emote::emote;
use crate::commands::fun::owoify::owoify;
use crate::commands::fun::qotd::qotd;
use crate::commands::fun::ship::ship;
use crate::commands::game::game;
use crate::commands::information::guide::guide;
use crate::commands::information::meal::meal;
use crate::commands::information::oracle::oracle;
use crate::commands::information::ping::ping;
use crate::commands::information::route::route;
use crate::commands::information::stats::stats;
use crate::commands::information::time::time;
use crate::commands::information::valentine::valentine;
use crate::commands::smite::smite;
use crate::commands::utility::avatar::avatar;
use crate::commands::utility::convert::convert;
use crate::commands::utility::enlarge::enlarge;
use crate::commands::utility::image::image;
use crate::commands::utility::pick::pick;
use crate::shared::services::openai_service::initialize_openai_client;
use crate::shared::structs::authentication::Authentication;
use crate::shared::structs::config::common_settings::initialize_common_settings;
use crate::shared::structs::config::server_info::initialize_server_infos;
use crate::shared::structs::fun::emote::initialize_emote_list;
use crate::shared::structs::fun::qotd::initialize_qotd_infos;
use crate::shared::structs::fun::ship_message::initialize_ship_messages;
use crate::shared::structs::game::quiz_question::initialize_quiz_questions;
use crate::shared::structs::information::character::{initialize_routes, initialize_valentines};
use crate::shared::structs::information::oracle::initialize_oracles;
use crate::shared::structs::smite::initialize_smite;
use crate::shared::structs::utility::convert::conversion_table::initialize_conversion_table;
use poise::FrameworkError;
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use serenity::all::{ChannelId, CreateAllowedMentions, GatewayIntents};
use shared::structs::config::*;
use shared::structs::record::*;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args()
        .map(|arg| arg.to_lowercase())
        .collect::<Vec<_>>();

    let channel_control = channel_control::initialize()?;
    let user_records = user_record::initialize()?;

    let enabled_channels = channel_control
        .enabled_channels
        .iter()
        .map(|n| ChannelId::from(*n))
        .collect::<HashSet<_>>();

    let kou = args.contains(&"kou".to_string());
    let config = configuration::initialize()?;
    let http_client = reqwest::Client::new();
    let openai_client = initialize_openai_client(&config, http_client.clone());

    let context_data = ContextData {
        config,
        kou,
        channel_control: Arc::new(RwLock::new(channel_control)),
        user_records: Arc::new(RwLock::new(user_records)),
        routes: initialize_routes(),
        valentines: initialize_valentines(),
        oracles: initialize_oracles(),
        http_client,
        conversion_table: initialize_conversion_table(),
        authentication: Arc::new(RwLock::new(Authentication::new())),
        emote_list: Arc::new(RwLock::new(initialize_emote_list()?)),
        server_infos: initialize_server_infos()?,
        qotd_infos: Arc::new(RwLock::new(initialize_qotd_infos()?)),
        common_settings: initialize_common_settings(kou)?,
        ship_messages: initialize_ship_messages(),
        quiz_questions: initialize_quiz_questions(kou),
        smite: initialize_smite()?,
        openai_client,
    };

    if context_data.config.token.is_empty() {
        return Err(anyhow::anyhow!("Discord token cannot be empty."));
    }

    let log_level = match context_data.config.log_level.as_str() {
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

    let token = context_data.config.token.clone();
    let prefix = context_data.config.prefix.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                about(),
                ping(),
                route(),
                valentine(),
                stats(),
                oracle(),
                meal(),
                avatar(),
                convert(),
                enlarge(),
                time(),
                image(),
                pick(),
                dialog(),
                emote(),
                owoify(),
                qotd(),
                ship(),
                admin(),
                game(),
                guide(),
                smite(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            command_check: None,
            allowed_mentions: Some(
                CreateAllowedMentions::new()
                    .all_roles(true)
                    .all_users(true)
                    .everyone(true),
            ),
            reply_callback: None,
            event_handler: (),
            initialize_owners: true,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(prefix),
                mention_as_prefix: false,
                execute_self_messages: false,
                ignore_bots: true,
                ignore_thread_creation: true,
                case_insensitive_commands: false,
                ..std::default::Default::default()
            },
            ..std::default::Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(context_data)
            })
        })
        .build();

    let intents = GatewayIntents::all();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    if let Err(e) = client.start().await {
        tracing::error!("Error when starting the bot: {}", e.to_string());
    }

    Ok(())
}

async fn on_error(framework_error: FrameworkError<'_, ContextData, ContextError>) {
    match framework_error {
        FrameworkError::Setup { error, .. } => {
            tracing::error!("Failed to start bot: {}", error.to_string());
        }
        FrameworkError::EventHandler { error, event, .. } => {
            tracing::error!(
                "Failed to handle event {}, error: {}",
                event.snake_case_name(),
                error.to_string()
            );
        }
        FrameworkError::Command { error, ctx, .. } => {
            let command_name = ctx.command().name.as_str();
            tracing::error!(
                "Failed to execute command {}, error: {}",
                command_name,
                error.to_string()
            );
        }
        FrameworkError::ArgumentParse { error, input, .. } => {
            tracing::error!(
                "Failed to parse argument {:?}, error: {}",
                &input,
                error.to_string()
            );
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ..
        } => {
            tracing::error!(
                "Bot is missing required permission: {:?}",
                missing_permissions
            )
        }
        FrameworkError::CommandCheckFailed { error, ctx, .. } => {
            let command_name = ctx.command().name.as_str();
            tracing::error!(
                "Command check failed, command: {}, error: {:?}",
                command_name,
                error
            )
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                tracing::error!("An uncategorized error occurred: {}", e.to_string());
            }
        }
    }
}
