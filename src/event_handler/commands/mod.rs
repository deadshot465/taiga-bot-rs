use once_cell::sync::OnceCell;
use serenity::builder::CreateApplicationCommands;
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
            "about".to_string(),
            crate::commands::information::about::about_async,
        );
        map.insert(
            "avatar".to_string(),
            crate::commands::utility::avatar::avatar_async,
        );
        map.insert(
            "enlarge".to_string(),
            crate::commands::utility::enlarge::enlarge_async,
        );
        map.insert("game".to_string(), crate::commands::game::dispatch_async);
        map.insert(
            "image".to_string(),
            crate::commands::utility::image::image_async,
        );
        map.insert(
            "meal".to_string(),
            crate::commands::information::meal::meal_async,
        );
        map.insert(
            "oracle".to_string(),
            crate::commands::information::oracle::oracle_async,
        );
        map.insert(
            "pick".to_string(),
            crate::commands::utility::pick::pick_async,
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

pub async fn build_global_slash_commands(
    ctx: &Context,
    force_recreate: bool,
) -> anyhow::Result<Vec<ApplicationCommand>> {
    if force_recreate {
        Ok(
            ApplicationCommand::set_global_application_commands(&ctx.http, |commands| commands)
                .await?,
        )
    } else {
        Ok(
            ApplicationCommand::set_global_application_commands(&ctx.http, |commands| commands)
                .await?,
        )
    }
}

pub async fn build_guild_slash_commands(ctx: &Context) -> anyhow::Result<Vec<ApplicationCommand>> {
    Ok(GuildId(KOU_SERVER_ID)
        .set_application_commands(&ctx.http, |commands| register_commands(commands))
        .await?)
}

fn register_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    register_about(commands);
    register_avatar(commands);
    register_enlarge(commands);
    register_game(commands);
    register_image(commands);
    register_meal(commands);
    register_oracle(commands);
    register_pick(commands);
    register_ping(commands);
    register_route(commands);
    register_stats(commands);
    register_valentine(commands)
}

fn register_about(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("about")
            .description("Shows information about the bot.")
    })
}

fn register_avatar(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("avatar")
            .description("Get avatar/profile image of yourself or another user.")
            .create_option(|opt| {
                opt.name("user")
                    .description("The user whose avatar to get.")
                    .required(true)
                    .kind(ApplicationCommandOptionType::User)
            })
    })
}

fn register_enlarge(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("enlarge")
            .description("Returns enlarged emote(s).")
            .create_option(|opt| {
                opt.kind(ApplicationCommandOptionType::String)
                    .name("emote")
                    .description("One or more emotes to enlarge.")
                    .required(true)
            })
    })
}

fn register_game(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("game")
            .description("Play mini games with Kou/Taiga.")
            .create_option(|opt| {
                opt.kind(ApplicationCommandOptionType::SubCommand)
                    .name("quiz")
                    .description(
                        "Play a fun quiz with your friends. Optionally specify rounds (default 7).",
                    )
                    .create_sub_option(|opt| {
                        opt.name("rounds")
                            .description("Rounds you want to play.")
                            .kind(ApplicationCommandOptionType::Integer)
                            .required(false)
                    })
            })
    })
}

fn register_image(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("image")
            .description("Get random images based on keywords.")
            .create_option(|opt| {
                opt.kind(ApplicationCommandOptionType::SubCommand)
                    .name("image")
                    .description("Get random images based on keywords.")
                    .create_sub_option(|opt| {
                        opt.name("keyword")
                            .description("Keyword to search for.")
                            .kind(ApplicationCommandOptionType::String)
                            .required(false)
                    })
            })
            .create_option(|opt| {
                opt.kind(ApplicationCommandOptionType::SubCommand)
                    .name("cat")
                    .description("Get cat images.")
                    .create_sub_option(|opt| {
                        opt.name("keyword")
                            .description("Keyword to search for.")
                            .kind(ApplicationCommandOptionType::String)
                            .required(false)
                    })
            })
            .create_option(|opt| {
                opt.kind(ApplicationCommandOptionType::SubCommand)
                    .name("dog")
                    .description("Get dog images.")
                    .create_sub_option(|opt| {
                        opt.name("keyword")
                            .description("Keyword to search for.")
                            .kind(ApplicationCommandOptionType::String)
                            .required(false)
                    })
            })
    })
}

fn register_meal(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(|cmd| cmd.name("meal").description("Get a random meal recipe."))
}

fn register_oracle(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("oracle")
            .description("Draw an oracle and know the future of something on your mind.")
    })
}

fn register_pick(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("pick")
            .description("Pick from several options.")
            .create_option(|opt| {
                opt.name("times")
                    .description(
                        "Times to pick. Negative numbers or numbers too big will be ignored.",
                    )
                    .kind(ApplicationCommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|opt| {
                opt.name("choices")
                    .description("Choices to pick from, separated by pipe (|).")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    })
}

fn register_ping(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("ping")
            .description("Returns latency and API ping.")
    })
}

fn register_route(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("route")
            .description("Tells you what route to play next.")
    })
}

fn register_stats(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("stats")
            .description("This command will show your records with several commands.")
            .create_option(|opt| {
                opt.required(false)
                    .name("command")
                    .description("(Optional) The command of which you want to query the record.")
                    .add_string_choice("route", "route")
                    .add_string_choice("valentine", "valentine")
                    .kind(ApplicationCommandOptionType::String)
            })
    })
}

fn register_valentine(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("valentine")
            .description("Tells you your next valentine.")
    })
}
