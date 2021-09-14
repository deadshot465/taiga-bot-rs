use once_cell::sync::OnceCell;
use serenity::model::prelude::application_command::{
    ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

const KOU_SERVER_ID: u64 = 705036924330704968;

pub type T = fn(
    Context,
    ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;

pub static AVAILABLE_COMMANDS: OnceCell<HashMap<String, T>> = OnceCell::new();

pub fn initialize() {
    AVAILABLE_COMMANDS.get_or_init(|| {
        let mut map: HashMap<String, T> = HashMap::new();
        map.insert(
            "meal".to_string(),
            crate::commands::information::meal::meal_async,
        );
        map.insert(
            "oracle".to_string(),
            crate::commands::information::oracle::oracle_async,
        );
        map.insert(
            "ping".to_string(),
            crate::commands::information::ping::ping_async,
        );
        map.insert(
            "route".to_string(),
            crate::commands::information::route::route_async,
        );
        map.insert(
            "stats".to_string(),
            crate::commands::information::stats::stats_async,
        );
        map.insert(
            "valentine".to_string(),
            crate::commands::information::valentine::valentine_async,
        );
        map
    });
}

pub async fn build_global_slash_commands(ctx: &Context) -> anyhow::Result<Vec<ApplicationCommand>> {
    Ok(ApplicationCommand::set_global_application_commands(&ctx.http, |commands| commands).await?)
}

pub async fn build_guild_slash_commands(ctx: &Context) -> anyhow::Result<Vec<ApplicationCommand>> {
    Ok(GuildId(KOU_SERVER_ID)
        .set_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|cmd| {
                    cmd.name("meal").description("Get a random meal recipe.")
                })
                .create_application_command(|cmd| {
                    cmd.name("oracle").description(
                        "Draw an oracle and know the future of something on your mind.",
                    )
                })
                .create_application_command(|cmd| {
                    cmd.name("ping")
                        .description("Returns latency and API ping.")
                })
                .create_application_command(|cmd| {
                    cmd.name("route")
                        .description("Tells you what route to play next.")
                })
                .create_application_command(|cmd| {
                    cmd.name("stats")
                        .description("This command will show your records with several commands.")
                        .create_option(|option| {
                            option
                                .required(false)
                                .name("command")
                                .description(
                                    "(Optional) The command of which you want to query the record.",
                                )
                                .add_string_choice("route", "route")
                                .add_string_choice("valentine", "valentine")
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_application_command(|cmd| {
                    cmd.name("valentine")
                        .description("Tells you your next valentine.")
                })
        })
        .await?)
}
