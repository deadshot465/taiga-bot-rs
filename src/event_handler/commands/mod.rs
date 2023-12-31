use crate::shared::constants::KOU_SERVER_ID;
use crate::shared::structs::config::server_info::SERVER_INFOS;
use once_cell::sync::Lazy;
use serenity::all::{CreateCommandPermission, RoleId};
use serenity::builder::{CreateCommand, CreateCommandOption, EditCommandPermissions};
use serenity::model::application::{Command, CommandInteraction, CommandOptionType};
use serenity::model::prelude::{CommandId, GuildId};
use serenity::model::Permissions;
use serenity::prelude::Context;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub const SKIP_CHANNEL_CHECK_COMMANDS: [&str; 3] = ["admin", "convert", "smite"];

pub type T =
    fn(Context, CommandInteraction) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;

#[derive(Clone)]
pub struct SlashCommandElements {
    pub handler: T,
    pub command: fn() -> CreateCommand,
    pub description: String,
    pub emoji: String,
}

pub static AVAILABLE_COMMANDS: Lazy<HashMap<String, SlashCommandElements>> = Lazy::new(initialize);

pub static GLOBAL_COMMANDS: Lazy<HashMap<String, SlashCommandElements>> = Lazy::new(|| {
    // Placeholder for testing with guild commands.
    let mut global_commands = AVAILABLE_COMMANDS.clone();
    global_commands.remove("qotd");
    global_commands
});

const ADMINISTRATIVE_COMMANDS: [&str; 2] = ["admin", "smite"];

pub fn initialize() -> HashMap<String, SlashCommandElements> {
    let mut map: HashMap<String, SlashCommandElements> = HashMap::new();
    map.insert(
        "about".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::about::about_async,
            command: register_about,
            description: "Shows information about the bot.".to_string(),
            emoji: "❓".to_string(),
        },
    );
    map.insert(
        "admin".to_string(),
        SlashCommandElements {
            handler: crate::commands::admin::dispatch_async,
            command: register_admin,
            description: "Administrative commands.".to_string(),
            emoji: "🚨".to_string(),
        },
    );
    map.insert(
        "avatar".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::avatar::avatar_async,
            command: register_avatar,
            description: "Get avatar/profile image of yourself or another user.".to_string(),
            emoji: "👤".to_string(),
        },
    );
    map.insert(
        "convert".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::convert::convert_async,
            command: register_convert,
            description: "Helps converting stuff.".to_string(),
            emoji: "🔄".to_string(),
        },
    );
    map.insert(
        "dialog".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::dialog::dialog_async,
            command: register_dialog,
            description: "Returns an image of a character saying anything you want.".to_string(),
            emoji: "💬".to_string(),
        },
    );
    map.insert(
        "emote".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::emote::emote_async,
            command: register_emote,
            description: "Add or remove an emote from the bot. Emotes from servers which the bot is not in won't work.".to_string(),
            emoji: "🤡".to_string()
        }
    );
    map.insert(
        "enlarge".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::enlarge::enlarge_async,
            command: register_enlarge,
            description: "Returns enlarged emote(s).".to_string(),
            emoji: "🔍".to_string(),
        },
    );
    map.insert(
        "game".to_string(),
        SlashCommandElements {
            handler: crate::commands::game::dispatch_async,
            command: register_game,
            description: "Play mini games with Kou/Taiga.".to_string(),
            emoji: "🎮".to_string(),
        },
    );
    map.insert(
        "guide".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::guide::guide_async,
            command: register_guide,
            description: "Start a step-by-step guide.".to_string(),
            emoji: "ℹ️".to_string(),
        },
    );
    map.insert(
        "image".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::image::image_async,
            command: register_image,
            description: "Get random images based on keywords.".to_string(),
            emoji: "🖼".to_string(),
        },
    );
    map.insert(
        "meal".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::meal::meal_async,
            command: register_meal,
            description: "Get a random meal recipe.".to_string(),
            emoji: "🍳".to_string(),
        },
    );
    map.insert(
        "oracle".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::oracle::oracle_async,
            command: register_oracle,
            description: "Draw an oracle and know the future of something on your mind."
                .to_string(),
            emoji: "🔮".to_string(),
        },
    );
    map.insert(
        "owoify".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::owoify::owoify_async,
            command: register_owoify,
            description: "This command will owoify your text.".to_string(),
            emoji: "👶".to_string(),
        },
    );
    map.insert(
        "pick".to_string(),
        SlashCommandElements {
            handler: crate::commands::utility::pick::pick_async,
            command: register_pick,
            description: "Pick from several options.".to_string(),
            emoji: "🔀".to_string(),
        },
    );
    map.insert(
        "ping".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::ping::ping_async,
            command: register_ping,
            description: "Returns latency and API ping.".to_string(),
            emoji: "🔔".to_string(),
        },
    );
    map.insert(
        "qotd".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::qotd::qotd_async,
            command: register_qotd,
            description: "Ask a question of the day and earn 25 credits.".to_string(),
            emoji: "🤔".to_string(),
        },
    );
    map.insert(
        "route".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::route::route_async,
            command: register_route,
            description: "Tells you what route to play next.".to_string(),
            emoji: "❤️".to_string(),
        },
    );
    map.insert(
        "ship".to_string(),
        SlashCommandElements {
            handler: crate::commands::fun::ship::ship_async,
            command: register_ship,
            description: "Ship two users.".to_string(),
            emoji: "🛳".to_string(),
        },
    );
    map.insert(
        "smite".to_string(),
        SlashCommandElements {
            handler: crate::commands::smite::smite_async,
            command: register_smite,
            description: "Smite bad behaving members.".to_string(),
            emoji: "⚡️".to_string(),
        },
    );
    map.insert(
        "stats".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::stats::stats_async,
            command: register_stats,
            description: "This command will show your records with several commands.".to_string(),
            emoji: "🧮".to_string(),
        },
    );
    map.insert(
        "time".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::time::time_async,
            command: register_time,
            description: "Query the time of a city based on a city name or an address.".to_string(),
            emoji: "🕐".to_string(),
        },
    );
    map.insert(
        "valentine".to_string(),
        SlashCommandElements {
            handler: crate::commands::information::valentine::valentine_async,
            command: register_valentine,
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
        Command::set_global_commands(&ctx.http, global_commands()).await?;
    } else {
        let global_commands = Command::get_global_commands(&ctx.http)
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
            for (
                _,
                SlashCommandElements {
                    command: register, ..
                },
            ) in commands_not_registered.into_iter()
            {
                Command::create_global_command(&ctx.http, register()).await?;
            }
        }
    }
    Ok(())
}

pub async fn build_guild_slash_commands(ctx: &Context) -> anyhow::Result<Vec<Command>> {
    Ok(GuildId::new(KOU_SERVER_ID)
        .set_commands(&ctx.http, guild_commands())
        .await?)
}

pub async fn set_commands_permission(ctx: &Context) -> anyhow::Result<()> {
    let global_commands = Command::get_global_commands(&ctx.http).await?;
    let commands = ADMINISTRATIVE_COMMANDS
        .iter()
        .map(|s| global_commands.iter().find(|cmd| cmd.name.as_str() == *s))
        .map(|opt| opt.map(|cmd| cmd.id))
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default();

    let guilds = SERVER_INFOS
        .server_infos
        .iter()
        .map(|info| (info.server_id, info.admin_role_ids.clone()));

    for (guild_id, role_ids) in guilds.into_iter() {
        if let Err(e) = set_permission(ctx, guild_id, &commands, &role_ids).await {
            tracing::error!("Error when setting permissions for commands: {}", e);
        }
    }

    Ok(())
}

async fn set_permission(
    ctx: &Context,
    guild_id: u64,
    cmds: &[CommandId],
    role_ids: &[u64],
) -> anyhow::Result<()> {
    let guild_id = GuildId::new(guild_id);
    for &cmd_id in cmds.iter() {
        for &role_id in role_ids.iter() {
            guild_id
                .edit_command_permissions(
                    &ctx.http,
                    cmd_id,
                    EditCommandPermissions::new(vec![CreateCommandPermission::role(
                        RoleId::new(role_id),
                        true,
                    )]),
                )
                .await?;
        }
    }

    Ok(())
}

fn global_commands() -> Vec<CreateCommand> {
    GLOBAL_COMMANDS
        .iter()
        .map(|(_, element)| (element.command)())
        .collect()
}

fn guild_commands() -> Vec<CreateCommand> {
    vec![register_qotd()]
}

fn register_about() -> CreateCommand {
    let description = get_command_description("about");
    CreateCommand::new("about").description(description)
}

fn register_admin() -> CreateCommand {
    let description = get_command_description("admin");

    CreateCommand::new("admin")
        .description(description)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "enable", "Enable a specific channel for bot usage.")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::Channel, "channel", "The channel to enable for bot usage.").required(true)))
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "disable", "Disable a specific channel for bot usage.")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::Channel, "channel", "The channel to disable for bot usage.").required(true)))
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "allow", "Allow a specific channel for random responses of bot.")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::Channel, "channel", "The channel to allow for random responses.").required(true)))
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "disallow", "Disallow a specific channel for random responses of bot.")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::Channel, "channel", "The channel to disallow for random responses.").required(true)))
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "purge", "Purge messages from this channel. Default to 10 most recent messages. Maximum 100 messages.")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::Integer, "amount", "The amount of messages to purge.").required(false)))
}

fn register_avatar() -> CreateCommand {
    let description = get_command_description("avatar");

    CreateCommand::new("avatar")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user whose avatar to get.",
            )
            .required(true),
        )
}

fn register_convert() -> CreateCommand {
    let description = get_command_description("convert");

    CreateCommand::new("convert")
        .description(description)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "length", "Convert length.")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "source_unit",
                        "The source length to convert from.",
                    )
                    .required(true)
                    .add_string_choice("km", "km")
                    .add_string_choice("m", "m")
                    .add_string_choice("cm", "cm")
                    .add_string_choice("inches", "in")
                    .add_string_choice("feet", "ft")
                    .add_string_choice("miles", "mi")
                    .add_string_choice("au", "au"),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "target_unit",
                        "The target length to convert to.",
                    )
                    .required(true)
                    .add_string_choice("km", "km")
                    .add_string_choice("m", "m")
                    .add_string_choice("cm", "cm")
                    .add_string_choice("inches", "in")
                    .add_string_choice("feet", "ft")
                    .add_string_choice("miles", "mi")
                    .add_string_choice("au", "au"),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Number,
                        "amount",
                        "The amount to convert.",
                    )
                    .required(true),
                ),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "weight", "Convert weight.")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "source_unit",
                        "The source weight to convert from.",
                    )
                    .required(true)
                    .add_string_choice("kg", "kg")
                    .add_string_choice("g", "g")
                    .add_string_choice("lb", "lb"),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "target_unit",
                        "The target weight to convert to.",
                    )
                    .required(true)
                    .add_string_choice("kg", "kg")
                    .add_string_choice("g", "g")
                    .add_string_choice("lb", "lb"),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Number,
                        "amount",
                        "The amount to convert.",
                    )
                    .required(true),
                ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "temperature",
                "Convert temperature.",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "source_unit",
                    "The source temperature to convert from.",
                )
                .required(true)
                .add_string_choice("Celsius", "c")
                .add_string_choice("Fahrenheit", "f")
                .add_string_choice("Kelvin", "k"),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "target_unit",
                    "The target temperature to convert to.",
                )
                .required(true)
                .add_string_choice("Celsius", "c")
                .add_string_choice("Fahrenheit", "f")
                .add_string_choice("Kelvin", "k"),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Number,
                    "amount",
                    "The amount to convert.",
                )
                .required(true),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "currency",
                "Convert currency.",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "source_unit",
                    "The source currency type to convert from, e.g. USD.",
                )
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "target_unit",
                    "The target currency type to convert to, e.g. JPY.",
                )
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Number,
                    "amount",
                    "The amount to convert.",
                )
                .required(true),
            ),
        )
}

fn register_dialog() -> CreateCommand {
    let description = get_command_description("dialog");

    CreateCommand::new("dialog")
        .description(description)
        .add_option(CreateCommandOption::new(CommandOptionType::String,"background","The background of the character. A random background if the specified one doesn't exist.").required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::String,"character","The character whom you want to make saying something.").required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::String,"text","The text of the dialog. Cannot be over 180 characters.").required(true))
}

fn register_emote() -> CreateCommand {
    let description = get_command_description("emote");

    CreateCommand::new("emote")
        .description(description)
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "list",
            "List registered emotes in this server.",
        ))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "add",
                "Add an emote to the emote list to be used with Kou/Taiga.",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "name",
                    "The name of the emote to be used with prefix.",
                )
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "emote",
                    "The emote to register.",
                )
                .required(true),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "remove",
                "Remove an emote from the emote list.",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "name",
                    "The name of the emote to be removed.",
                )
                .required(true),
            ),
        )
}

fn register_enlarge() -> CreateCommand {
    let description = get_command_description("enlarge");

    CreateCommand::new("enlarge")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "emote",
                "One or more emotes to enlarge.",
            )
            .required(true),
        )
}

fn register_game() -> CreateCommand {
    let description = get_command_description("game");

    CreateCommand::new("game")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "quiz",
                "Play a fun quiz with your friends. Optionally specify rounds (default 7).",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "rounds",
                    "Rounds you want to play.",
                )
                .required(false),
            ),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "hangman",
            "Play a hangman game with Taiga or Kou.",
        ))
}

fn register_guide() -> CreateCommand {
    let description = get_command_description("guide");
    CreateCommand::new("guide").description(description)
}

fn register_image() -> CreateCommand {
    let description = get_command_description("image");

    CreateCommand::new("image")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "image",
                "Get random images based on keywords.",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "keyword",
                    "Keyword to search for.",
                )
                .required(false),
            ),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "cat", "Get cat images.")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "keyword",
                        "Keyword to search for.",
                    )
                    .required(false),
                ),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "dog", "Get dog images.")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "keyword",
                        "Keyword to search for.",
                    )
                    .required(false),
                ),
        )
}

fn register_meal() -> CreateCommand {
    let description = get_command_description("meal");
    CreateCommand::new("meal").description(description)
}

fn register_oracle() -> CreateCommand {
    let description = get_command_description("oracle");
    CreateCommand::new("oracle").description(description)
}

fn register_owoify() -> CreateCommand {
    let description = get_command_description("owoify");

    CreateCommand::new("owoify")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "level",
                "The owoness you want to owoify your text.",
            )
            .required(true)
            .add_string_choice("soft", "soft")
            .add_string_choice("medium", "medium")
            .add_string_choice("hard", "hard"),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "text", "The text to owoify.")
                .required(true),
        )
}

fn register_pick() -> CreateCommand {
    let description = get_command_description("pick");

    CreateCommand::new("pick")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "times",
                "Times to pick. Negative numbers or numbers too big will be ignored.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choices",
                "Choices to pick from, separated by pipe (|).",
            )
            .required(true),
        )
}

fn register_ping() -> CreateCommand {
    let description = get_command_description("ping");
    CreateCommand::new("ping").description(description)
}

fn register_qotd() -> CreateCommand {
    let description = get_command_description("qotd");

    CreateCommand::new("qotd")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "question",
                "The question of the day to ask.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Attachment,
                "attachment",
                "The attachment to add to the question of the day.",
            )
            .required(false),
        )
}

fn register_route() -> CreateCommand {
    let description = get_command_description("route");
    CreateCommand::new("route").description(description)
}

fn register_smite() -> CreateCommand {
    let description = get_command_description("smite");
    CreateCommand::new("smite")
        .description(description)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "member",
                "Bad behaving member to smite.",
            )
            .required(true),
        )
}

fn register_ship() -> CreateCommand {
    let description = get_command_description("ship");

    CreateCommand::new("ship")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user_1",
                "The first user to ship with the second user.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user_2",
                "The second user to ship with the first user.",
            )
            .required(true),
        )
}

fn register_stats() -> CreateCommand {
    let description = get_command_description("stats");

    CreateCommand::new("stats")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "command",
                "(Optional) The command of which you want to query the record.",
            )
            .required(false)
            .add_string_choice("route", "route")
            .add_string_choice("valentine", "valentine"),
        )
}

fn register_time() -> CreateCommand {
    let description = get_command_description("time");

    CreateCommand::new("time")
        .description(description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "city_name_or_address",
                "A city name or an address of which to query time.",
            )
            .required(true),
        )
}

fn register_valentine() -> CreateCommand {
    let description = get_command_description("valentine");
    CreateCommand::new("valentine").description(description)
}

fn get_command_description(name: &str) -> &str {
    AVAILABLE_COMMANDS
        .get(name)
        .map(|SlashCommandElements { description, .. }| description.as_str())
        .unwrap_or_default()
}
