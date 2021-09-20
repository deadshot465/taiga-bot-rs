use crate::shared::constants::{
    KOU_SERVER_ADMIN_ROLE_ID, KOU_SERVER_ID, TAIGA_SERVER_ADMIN_ROLE_ID, TAIGA_SERVER_ID,
    TAIGA_SERVER_WINTER_SPLENDOR_ROLE_ID,
};
use once_cell::sync::Lazy;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommands};
use serenity::model::interactions::application_command::ApplicationCommandPermissionType;
use serenity::model::prelude::application_command::{
    ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub type T = fn(
    Context,
    ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;

#[derive(Clone)]
pub struct SlashCommandElements {
    pub handler: T,
    pub register: fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    pub description: String,
    pub emoji: String,
}

pub static AVAILABLE_COMMANDS: Lazy<HashMap<String, SlashCommandElements>> = Lazy::new(initialize);

pub static GLOBAL_COMMANDS: Lazy<HashMap<String, SlashCommandElements>> = Lazy::new(|| {
    let mut global_commands = AVAILABLE_COMMANDS.clone();
    global_commands.remove("smite");
    global_commands
});

pub fn initialize() -> HashMap<String, SlashCommandElements> {
    let mut map: HashMap<String, SlashCommandElements> = HashMap::new();
    map.insert(
        "about".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::about::about_async,
            register: register_about,
            description: "Shows information about the bot.".to_string(),
            emoji: "❓".to_string(),
        },
    );
    map.insert(
        "admin".to_string(),
        SlashCommandElements {
            handler: crate::commands::admin::dispatch_async,
            register: register_admin,
            description: "Administrative commands.".to_string(),
            emoji: "🚨".to_string(),
        },
    );
    map.insert(
        "avatar".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::avatar::avatar_async,
            register: register_avatar,
            description: "Get avatar/profile image of yourself or another user.".to_string(),
            emoji: "👤".to_string(),
        },
    );
    map.insert(
        "convert".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::convert::convert_async,
            register: register_convert,
            description: "Helps converting stuff.".to_string(),
            emoji: "🔄".to_string(),
        },
    );
    map.insert(
        "dialog".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::dialog::dialog_async,
            register: register_dialog,
            description: "Returns an image of a character saying anything you want.".to_string(),
            emoji: "💬".to_string(),
        },
    );
    map.insert(
        "enlarge".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::enlarge::enlarge_async,
            register: register_enlarge,
            description: "Returns enlarged emote(s).".to_string(),
            emoji: "🔍".to_string(),
        },
    );
    map.insert(
        "game".to_string(),
        SlashCommandElements {
            handler: crate::commands::game::dispatch_async,
            register: register_game,
            description: "Play mini games with Kou/Taiga.".to_string(),
            emoji: "🎮".to_string(),
        },
    );
    map.insert(
        "guide".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::guide::guide_async,
            register: register_guide,
            description: "Start a step-by-step guide.".to_string(),
            emoji: "ℹ️".to_string(),
        },
    );
    map.insert(
        "image".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::image::image_async,
            register: register_image,
            description: "Get random images based on keywords.".to_string(),
            emoji: "🖼".to_string(),
        },
    );
    map.insert(
        "meal".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::meal::meal_async,
            register: register_meal,
            description: "Get a random meal recipe.".to_string(),
            emoji: "🍳".to_string(),
        },
    );
    map.insert(
        "oracle".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::oracle::oracle_async,
            register: register_oracle,
            description: "Draw an oracle and know the future of something on your mind."
                .to_string(),
            emoji: "🔮".to_string(),
        },
    );
    map.insert(
        "owoify".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::owoify::owoify_async,
            register: register_owoify,
            description: "This command will owoify your text.".to_string(),
            emoji: "👶".to_string(),
        },
    );
    map.insert(
        "pick".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::pick::pick_async,
            register: register_pick,
            description: "Pick from several options.".to_string(),
            emoji: "🔀".to_string(),
        },
    );
    map.insert(
        "ping".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::ping::ping_async,
            register: register_ping,
            description: "Returns latency and API ping.".to_string(),
            emoji: "🔔".to_string(),
        },
    );
    map.insert(
        "route".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::route::route_async,
            register: register_route,
            description: "Tells you what route to play next.".to_string(),
            emoji: "❤️".to_string(),
        },
    );
    map.insert(
        "ship".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::ship::ship_async,
            register: register_ship,
            description: "Ship two users.".to_string(),
            emoji: "🛳".to_string(),
        },
    );
    map.insert(
        "smite".to_string(),
        SlashCommandElements {
            handler: crate::commands::smite::smite_async,
            register: register_smite,
            description: "Smite bad behaving members.".to_string(),
            emoji: "⚡️".to_string(),
        },
    );
    map.insert(
        "stats".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::stats::stats_async,
            register: register_stats,
            description: "This command will show your records with several commands.".to_string(),
            emoji: "🧮".to_string(),
        },
    );
    map.insert(
        "time".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::time::time_async,
            register: register_time,
            description: "Query the time of a city based on a city name or an address.".to_string(),
            emoji: "🕐".to_string(),
        },
    );
    map.insert(
        "valentine".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::valentine::valentine_async,
            register: register_valentine,
            description: "Tells you your next valentine.".to_string(),
            emoji: "💘".to_string(),
        },
    );
    map
}

pub async fn build_global_slash_commands(
    ctx: &Context,
    force_recreate: bool,
) -> anyhow::Result<()> {
    if force_recreate {
        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            register_global_commands(commands)
        })
        .await?;
    } else {
        let global_commands = ApplicationCommand::get_global_application_commands(&ctx.http)
            .await?
            .into_iter()
            .map(|cmd| cmd.name)
            .collect::<Vec<_>>();
        let commands_not_registered = GLOBAL_COMMANDS
            .iter()
            .filter(|(name, _)| !global_commands.contains(*name))
            .collect::<Vec<_>>();

        let has_unregistered_commands = !commands_not_registered.is_empty();

        if has_unregistered_commands {
            for (_, SlashCommandElements { register, .. }) in commands_not_registered.into_iter() {
                ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                    register(command)
                })
                .await?;
            }
        }
    }
    Ok(())
}

pub async fn build_guild_slash_commands(ctx: &Context) -> anyhow::Result<Vec<ApplicationCommand>> {
    Ok(GuildId(KOU_SERVER_ID)
        .set_application_commands(&ctx.http, |commands| register_guild_commands(commands))
        .await?)
}

pub async fn set_commands_permission(ctx: &Context) -> anyhow::Result<()> {
    let global_commands = ApplicationCommand::get_global_application_commands(&ctx.http).await?;
    let admin_command = global_commands
        .iter()
        .find(|cmd| cmd.name.as_str() == "admin");

    if let Some(cmd) = admin_command {
        let guilds = vec![
            (KOU_SERVER_ID, KOU_SERVER_ADMIN_ROLE_ID),
            (TAIGA_SERVER_ID, TAIGA_SERVER_ADMIN_ROLE_ID),
        ];

        for (server_id, role_id) in guilds.into_iter() {
            set_permission(ctx, server_id, cmd.id.0, role_id).await?;
        }
    }

    let smite_command = global_commands
        .iter()
        .find(|cmd| cmd.name.as_str() == "smite");
    if let Some(cmd) = smite_command {
        let guilds = vec![
            (KOU_SERVER_ID, KOU_SERVER_ADMIN_ROLE_ID),
            (TAIGA_SERVER_ID, TAIGA_SERVER_ADMIN_ROLE_ID),
            (TAIGA_SERVER_ID, TAIGA_SERVER_WINTER_SPLENDOR_ROLE_ID),
        ];

        for (server_id, role_id) in guilds.into_iter() {
            set_permission(ctx, server_id, cmd.id.0, role_id).await?;
        }
    }

    Ok(())
}

async fn set_permission(
    ctx: &Context,
    guild_id: u64,
    command_id: u64,
    admin_role_id: u64,
) -> anyhow::Result<()> {
    GuildId(guild_id)
        .set_application_commands_permissions(&ctx.http, |permissions| {
            permissions.create_application_command(|permission| {
                permission.id(command_id).create_permissions(|data| {
                    data.kind(ApplicationCommandPermissionType::Role)
                        .permission(true)
                        .id(admin_role_id)
                })
            })
        })
        .await?;

    Ok(())
}

fn register_global_commands(
    commands: &mut CreateApplicationCommands,
) -> &mut CreateApplicationCommands {
    for (_, SlashCommandElements { register, .. }) in GLOBAL_COMMANDS.iter() {
        commands.create_application_command(|command| register(command));
    }
    commands
}

fn register_guild_commands(
    commands: &mut CreateApplicationCommands,
) -> &mut CreateApplicationCommands {
    commands.create_application_command(|command| register_smite(command))
}

fn register_about(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("about");
    cmd.name("about").description(description)
}

fn register_admin(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("admin");

    cmd.name("admin")
        .description(description)
        .default_permission(false)
        .create_option(|opt| {
            opt.name("enable")
                .description("Enable a specific channel for bot usage.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("channel")
                        .description("The channel to enable for bot usage.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::Channel)
                })
        })
        .create_option(|opt| {
            opt.name("disable")
                .description("Disable a specific channel for bot usage.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("channel")
                        .description("The channel to disable for bot usage.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::Channel)
                })
        })
        .create_option(|opt| {
            opt.name("allow")
                .description("Allow a specific channel for random responses of bot.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("channel")
                        .description("The channel to allow for random responses.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::Channel)
                })
        })
        .create_option(|opt| {
            opt.name("disallow")
                .description("Disallow a specific channel for random responses of bot.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("channel")
                        .description("The channel to disallow for random responses.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::Channel)
                })
        })
        .create_option(|opt| {
            opt.name("purge")
                .description("Purge messages from this channel. Default to 10 most recent messages. Maximum 100 messages.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("amount")
                        .description("The number of messages to purge.")
                        .required(false)
                        .kind(ApplicationCommandOptionType::Integer)
                })
        })
}

fn register_avatar(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("avatar");

    cmd.name("avatar")
        .description(description)
        .create_option(|opt| {
            opt.name("user")
                .description("The user whose avatar to get.")
                .required(true)
                .kind(ApplicationCommandOptionType::User)
        })
}

fn register_convert(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("convert");

    cmd.name("convert")
        .description(description)
        .create_option(|opt| {
            opt.name("length")
                .description("Convert length.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("source_unit")
                        .description("The source length to convert from.")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                        .add_string_choice("km", "km")
                        .add_string_choice("m", "m")
                        .add_string_choice("cm", "cm")
                        .add_string_choice("inches", "in")
                        .add_string_choice("feet", "ft")
                        .add_string_choice("miles", "mi")
                        .add_string_choice("au", "au")
                })
                .create_sub_option(|opt| {
                    opt.name("target_unit")
                        .description("The target length to convert to.")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                        .add_string_choice("km", "km")
                        .add_string_choice("m", "m")
                        .add_string_choice("cm", "cm")
                        .add_string_choice("inches", "in")
                        .add_string_choice("feet", "ft")
                        .add_string_choice("miles", "mi")
                        .add_string_choice("au", "au")
                })
                .create_sub_option(|opt| {
                    opt.name("amount")
                        .description("The amount to convert.")
                        .kind(ApplicationCommandOptionType::Number)
                        .required(true)
                })
        })
        .create_option(|opt| {
            opt.name("weight")
                .description("Convert weight.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("source_unit")
                        .description("The source weight to convert from.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                        .add_string_choice("kg", "kg")
                        .add_string_choice("g", "g")
                        .add_string_choice("lb", "lb")
                })
                .create_sub_option(|opt| {
                    opt.name("target_unit")
                        .description("The target weight to convert to.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                        .add_string_choice("kg", "kg")
                        .add_string_choice("g", "g")
                        .add_string_choice("lb", "lb")
                })
                .create_sub_option(|opt| {
                    opt.name("amount")
                        .description("The amount to convert.")
                        .kind(ApplicationCommandOptionType::Number)
                        .required(true)
                })
        })
        .create_option(|opt| {
            opt.name("temperature")
                .description("Convert temperature.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("source_unit")
                        .description("The source temperature to convert from.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                        .add_string_choice("Celsius", "c")
                        .add_string_choice("Fahrenheit", "f")
                        .add_string_choice("Kelvin", "k")
                })
                .create_sub_option(|opt| {
                    opt.name("target_unit")
                        .description("The target temperature to convert to.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                        .add_string_choice("Celsius", "c")
                        .add_string_choice("Fahrenheit", "f")
                        .add_string_choice("Kelvin", "k")
                })
                .create_sub_option(|opt| {
                    opt.name("amount")
                        .description("The amount to convert.")
                        .kind(ApplicationCommandOptionType::Number)
                        .required(true)
                })
        })
        .create_option(|opt| {
            opt.name("currency")
                .description("Convert currency.")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("source_unit")
                        .description("The source currency type to convert from, e.g. USD.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                })
                .create_sub_option(|opt| {
                    opt.name("target_unit")
                        .description("The target currency type to convert to, e.g. JPY.")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                })
                .create_sub_option(|opt| {
                    opt.name("amount")
                        .description("The amount to convert.")
                        .kind(ApplicationCommandOptionType::Number)
                        .required(true)
                })
        })
}

fn register_dialog(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("dialog");

    cmd.name("dialog")
        .description(description)
        .create_option(|opt| {
            opt.name("background")
                .description("The background of the character. A random background if the specified one doesn't exist.")
                .required(true)
                .kind(ApplicationCommandOptionType::String)
        })
        .create_option(|opt| {
            opt.name("character")
                .description("The character whom you want to make saying something.")
                .required(true)
                .kind(ApplicationCommandOptionType::String)
        })
        .create_option(|opt| {
            opt.name("text")
                .description("The text of the dialog. Cannot be over 180 characters.")
                .required(true)
                .kind(ApplicationCommandOptionType::String)
        })
}

fn register_enlarge(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("enlarge");

    cmd.name("enlarge")
        .description(description)
        .create_option(|opt| {
            opt.kind(ApplicationCommandOptionType::String)
                .name("emote")
                .description("One or more emotes to enlarge.")
                .required(true)
        })
}

fn register_game(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("game");

    cmd.name("game")
        .description(description)
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
        .create_option(|opt| {
            opt.kind(ApplicationCommandOptionType::SubCommand)
                .name("hangman")
                .description("Play a hangman game with Taiga or Kou.")
        })
}

fn register_guide(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("guide");
    cmd.name("guide").description(description)
}

fn register_image(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("image");

    cmd.name("image")
        .description(description)
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
}

fn register_meal(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("meal");
    cmd.name("meal").description(description)
}

fn register_oracle(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("oracle");
    cmd.name("oracle").description(description)
}

fn register_owoify(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("owoify");

    cmd.name("owoify")
        .description(description)
        .create_option(|opt| {
            opt.kind(ApplicationCommandOptionType::String)
                .name("level")
                .description("The owoiness you want to owoify your text.")
                .required(true)
                .add_string_choice("soft", "soft")
                .add_string_choice("medium", "medium")
                .add_string_choice("hard", "hard")
        })
        .create_option(|opt| {
            opt.kind(ApplicationCommandOptionType::String)
                .name("text")
                .description("The text to owoify.")
                .required(true)
        })
}

fn register_pick(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("pick");

    cmd.name("pick")
        .description(description)
        .create_option(|opt| {
            opt.name("times")
                .description("Times to pick. Negative numbers or numbers too big will be ignored.")
                .kind(ApplicationCommandOptionType::Integer)
                .required(true)
        })
        .create_option(|opt| {
            opt.name("choices")
                .description("Choices to pick from, separated by pipe (|).")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
}

fn register_ping(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("ping");
    cmd.name("ping").description(description)
}

fn register_route(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("route");
    cmd.name("route").description(description)
}

fn register_smite(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("smite");
    cmd.name("smite")
        .description(description)
        .default_permission(false)
        .create_option(|opt| {
            opt.name("member")
                .description("Bad behaving member to smite.")
                .kind(ApplicationCommandOptionType::User)
                .required(true)
        })
}

fn register_ship(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("ship");

    cmd.name("ship")
        .description(description)
        .create_option(|opt| {
            opt.required(true)
                .name("user_1")
                .description("The first user to ship with the second user.")
                .kind(ApplicationCommandOptionType::User)
        })
        .create_option(|opt| {
            opt.required(true)
                .name("user_2")
                .description("The second user to ship with the first user.")
                .kind(ApplicationCommandOptionType::User)
        })
}

fn register_stats(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("stats");

    cmd.name("stats")
        .description(description)
        .create_option(|opt| {
            opt.required(false)
                .name("command")
                .description("(Optional) The command of which you want to query the record.")
                .add_string_choice("route", "route")
                .add_string_choice("valentine", "valentine")
                .kind(ApplicationCommandOptionType::String)
        })
}

fn register_time(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("time");

    cmd.name("time")
        .description(description)
        .create_option(|opt| {
            opt.name("city_name_or_address")
                .description("A city name or an address of which to query time.")
                .required(true)
                .kind(ApplicationCommandOptionType::String)
        })
}

fn register_valentine(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let description = get_command_description("valentine");
    cmd.name("valentine").description(description)
}

fn get_command_description(name: &str) -> &str {
    AVAILABLE_COMMANDS
        .get(name)
        .map(|SlashCommandElements { description, .. }| description.as_str())
        .unwrap_or_default()
}
