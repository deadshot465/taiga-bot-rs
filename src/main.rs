use std::sync::Arc;

use poise::{serenity_prelude as serenity, BoxFuture, PrefixFrameworkOptions};
use poise::{CreateReply, FrameworkError};
use serenity::all::{CreateAllowedMentions, GatewayIntents};
use tokio::sync::RwLock;
use tracing::Level;

use shared::structs::config::*;
use shared::structs::record::*;

use crate::event_handler::handle_event;
use crate::shared::constants::CONFIG_DIRECTORY;
use crate::shared::services::open_router_service::initialize_open_router_client;
use crate::shared::services::openai_service::initialize_openai_client;
use crate::shared::structs::authentication::Authentication;
use crate::shared::structs::config::common_settings::initialize_common_settings;
use crate::shared::structs::config::random_response::initialize_random_response;
use crate::shared::structs::config::server_info::initialize_server_infos;
use crate::shared::structs::fun::emote::initialize_emote_list;
use crate::shared::structs::fun::qotd::initialize_qotd_infos;
use crate::shared::structs::fun::ship_message::initialize_ship_messages;
use crate::shared::structs::game::quiz_question::initialize_quiz_questions;
use crate::shared::structs::information::character::{initialize_routes, initialize_valentines};
use crate::shared::structs::information::oracle::initialize_oracles;
use crate::shared::structs::smite::initialize_smite;
use crate::shared::structs::utility::convert::conversion_table::initialize_conversion_table;
use crate::shared::structs::{Context, ContextData, ContextError};

mod commands;
mod event_handler;
mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args()
        .map(|arg| arg.to_lowercase())
        .collect::<Vec<_>>();

    let channel_control = channel_control::initialize()?;
    let user_records = user_record::initialize()?;

    let kou = args.contains(&"kou".to_string());
    let config = configuration::initialize()?;
    let http_client = reqwest::Client::new();
    let openai_client = initialize_openai_client(&config);
    let open_router_client = initialize_open_router_client(&config);

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
        random_response: initialize_random_response()?,
        translation_instructions: load_translation_instruction()?,
        open_router_client,
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
                commands::information::about::about(),
                commands::information::ping::ping(),
                commands::information::route::route(),
                commands::information::valentine::valentine(),
                commands::information::stats::stats(),
                commands::information::oracle::oracle(),
                commands::information::meal::meal(),
                commands::utility::avatar::avatar(),
                commands::utility::convert::convert(),
                commands::utility::enlarge::enlarge(),
                commands::information::time::time(),
                commands::utility::image::image(),
                commands::utility::pick::pick(),
                commands::fun::dialog::dialog(),
                commands::fun::emote::emote(),
                commands::fun::owoify::owoify(),
                commands::fun::qotd::qotd(),
                commands::fun::ship::ship(),
                commands::admin::admin(),
                commands::game::game(),
                commands::information::guide::guide(),
                commands::smite::smite(),
                commands::utility::save_file::save_file(),
                commands::fun::answer_anon::answer_anon(),
                commands::utility::translate::translate(),
                commands::fun::what_do_you_think::what_do_you_think(),
                commands::utility::batch_translate::batch_translate(),
            ],
            on_error: |error| Box::pin(handle_error(error)),
            command_check: Some(check_command),
            allowed_mentions: Some(
                CreateAllowedMentions::new()
                    .all_roles(true)
                    .all_users(true)
                    .everyone(true),
            ),
            reply_callback: None,
            event_handler: |ctx, event, framework, data| {
                Box::pin(handle_event(ctx, event, framework, data))
            },
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
        .setup(|ctx, _ready, framework| {
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

async fn handle_error(framework_error: FrameworkError<'_, ContextData, ContextError>) {
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
            let result = ctx.send(CreateReply::default().content("Either this channel is not enabled for commands, or you're not calling in a guild!")).await;

            if let Ok(reply_handle) = result {
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                if let Err(e) = reply_handle.delete(ctx).await {
                    tracing::error!(
                        "Error when deleting original response to slash command: {}",
                        e
                    );
                }
            }

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

const SKIP_CHECK_COMMANDS: [&str; 12] = [
    "save_file",
    "answer_anon",
    "enable",
    "disable",
    "allow",
    "disallow",
    "purge",
    "convert",
    "length",
    "weight",
    "temperature",
    "currency",
];

fn check_command(ctx: Context<'_>) -> BoxFuture<'_, Result<bool, ContextError>> {
    Box::pin(check_command_async(ctx))
}

async fn check_command_async(ctx: Context<'_>) -> Result<bool, ContextError> {
    let channel_id = ctx.channel_id();
    let command_name = ctx.command().name.as_str();
    let channel_control = ctx.data().channel_control.read().await;
    Ok(SKIP_CHECK_COMMANDS.contains(&command_name)
        || channel_control.enabled_channels.contains(&channel_id.get()))
}

fn load_translation_instruction() -> anyhow::Result<String> {
    if !std::path::Path::new(CONFIG_DIRECTORY).exists() {
        std::fs::create_dir(CONFIG_DIRECTORY)?
    }

    let translation_instructions_path =
        String::from(CONFIG_DIRECTORY) + "/translation_instructions.txt";
    if !std::path::Path::new(&translation_instructions_path).exists() {
        Ok("".to_string())
    } else {
        let instructions = std::fs::read_to_string(translation_instructions_path)?;
        Ok(instructions)
    }
}
