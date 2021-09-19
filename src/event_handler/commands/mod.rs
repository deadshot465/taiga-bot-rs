use once_cell::sync::OnceCell;
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

const KOU_SERVER_ID: u64 = 705036924330704968;
const KOU_SERVER_ADMIN_ROLE_ID: u64 = 706778860812894228;

const TAIGA_SERVER_ID: u64 = 696414250406510623;
const TAIGA_SERVER_ADMIN_ROLE_ID: u64 = 742061690824294520;

pub type T = fn(
    Context,
    ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;

pub static AVAILABLE_COMMANDS: OnceCell<
    HashMap<
        String,
        (
            T,
            fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
        ),
    >,
> = OnceCell::new();

pub static GLOBAL_COMMANDS: OnceCell<
    HashMap<
        String,
        (
            T,
            fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
        ),
    >,
> = OnceCell::new();

pub fn initialize() {
    AVAILABLE_COMMANDS.get_or_init(|| {
        let mut map: HashMap<
            String,
            (
                T,
                fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
            ),
        > = HashMap::new();
        map.insert(
            "about".to_string(),
            (
                crate::commands::information::about::about_async,
                register_about,
            ),
        );
        map.insert(
            "admin".to_string(),
            (crate::commands::admin::dispatch_async, register_admin),
        );
        map.insert(
            "avatar".to_string(),
            (
                crate::commands::utility::avatar::avatar_async,
                register_avatar,
            ),
        );
        map.insert(
            "convert".to_string(),
            (
                crate::commands::utility::convert::convert_async,
                register_convert,
            ),
        );
        map.insert(
            "dialog".to_string(),
            (crate::commands::fun::dialog::dialog_async, register_dialog),
        );
        map.insert(
            "enlarge".to_string(),
            (
                crate::commands::utility::enlarge::enlarge_async,
                register_enlarge,
            ),
        );
        map.insert(
            "game".to_string(),
            (crate::commands::game::dispatch_async, register_game),
        );
        map.insert(
            "image".to_string(),
            (crate::commands::utility::image::image_async, register_image),
        );
        map.insert(
            "meal".to_string(),
            (
                crate::commands::information::meal::meal_async,
                register_meal,
            ),
        );
        map.insert(
            "oracle".to_string(),
            (
                crate::commands::information::oracle::oracle_async,
                register_oracle,
            ),
        );
        map.insert(
            "owoify".to_string(),
            (crate::commands::fun::owoify::owoify_async, register_owoify),
        );
        map.insert(
            "pick".to_string(),
            (crate::commands::utility::pick::pick_async, register_pick),
        );
        map.insert(
            "ping".to_string(),
            (
                crate::commands::information::ping::ping_async,
                register_ping,
            ),
        );
        map.insert(
            "route".to_string(),
            (
                crate::commands::information::route::route_async,
                register_route,
            ),
        );
        map.insert(
            "ship".to_string(),
            (crate::commands::fun::ship::ship_async, register_ship),
        );
        map.insert(
            "stats".to_string(),
            (
                crate::commands::information::stats::stats_async,
                register_stats,
            ),
        );
        map.insert(
            "time".to_string(),
            (
                crate::commands::information::time::time_async,
                register_time,
            ),
        );
        map.insert(
            "valentine".to_string(),
            (
                crate::commands::information::valentine::valentine_async,
                register_valentine,
            ),
        );
        map
    });

    GLOBAL_COMMANDS.get_or_init(|| {
        let available_commands = AVAILABLE_COMMANDS
            .get()
            .expect("Failed to get available commands.");
        let mut global_commands = available_commands.clone();
        global_commands.remove("game");
        global_commands
    });
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
            .get()
            .expect("Failed to get available commands")
            .iter()
            .filter(|(name, _)| !global_commands.contains(*name))
            .collect::<Vec<_>>();

        let has_unregistered_commands = !commands_not_registered.is_empty();

        if has_unregistered_commands {
            for (_, (_, register)) in commands_not_registered.into_iter() {
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

pub async fn set_admin_commands_permission(ctx: &Context) -> anyhow::Result<()> {
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
    let global_commands = GLOBAL_COMMANDS
        .get()
        .expect("Failed to get globally available commands.");
    for (_, (_, register)) in global_commands.iter() {
        commands.create_application_command(|command| register(command));
    }
    commands
}

fn register_guild_commands(
    commands: &mut CreateApplicationCommands,
) -> &mut CreateApplicationCommands {
    commands.create_application_command(|command| register_game(command))
}

fn register_about(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("about")
        .description("Shows information about the bot.")
}

fn register_admin(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("admin")
        .description("Administrative commands.")
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
    cmd.name("avatar")
        .description("Get avatar/profile image of yourself or another user.")
        .create_option(|opt| {
            opt.name("user")
                .description("The user whose avatar to get.")
                .required(true)
                .kind(ApplicationCommandOptionType::User)
        })
}

fn register_convert(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("convert")
        .description("Helps converting stuff.")
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
    cmd.name("dialog")
        .description("Returns an image of a character saying anything you want.")
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
    cmd.name("enlarge")
        .description("Returns enlarged emote(s).")
        .create_option(|opt| {
            opt.kind(ApplicationCommandOptionType::String)
                .name("emote")
                .description("One or more emotes to enlarge.")
                .required(true)
        })
}

fn register_game(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
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
}

fn register_image(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
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
}

fn register_meal(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("meal").description("Get a random meal recipe.")
}

fn register_oracle(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("oracle")
        .description("Draw an oracle and know the future of something on your mind.")
}

fn register_owoify(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("owoify")
        .description("This command will owoify your text.")
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
    cmd.name("pick")
        .description("Pick from several options.")
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
    cmd.name("ping")
        .description("Returns latency and API ping.")
}

fn register_route(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("route")
        .description("Tells you what route to play next.")
}

fn register_ship(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("ship")
        .description("Ship two users.")
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
}

fn register_time(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("time")
        .description("Query the time of a city based on a city name or an address.")
        .create_option(|opt| {
            opt.name("city_name_or_address")
                .description("A city name or an address of which to query time.")
                .required(true)
                .kind(ApplicationCommandOptionType::String)
        })
}

fn register_valentine(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("valentine")
        .description("Tells you your next valentine.")
}
