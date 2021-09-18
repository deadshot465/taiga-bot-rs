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
            "convert".to_string(),
            crate::commands::utility::convert::convert_async,
        );
        map.insert(
            "dialog".to_string(),
            crate::commands::fun::dialog::dialog_async,
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
            "owoify".to_string(),
            crate::commands::fun::owoify::owoify_async,
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
        map.insert("ship".to_string(), crate::commands::fun::ship::ship_async);
        map.insert(
            "stats".to_string(),
            crate::commands::information::stats::stats_async,
        );
        map.insert(
            "time".to_string(),
            crate::commands::information::time::time_async,
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
    register_convert(commands);
    register_dialog(commands);
    register_enlarge(commands);
    register_game(commands);
    register_image(commands);
    register_meal(commands);
    register_oracle(commands);
    register_owoify(commands);
    register_pick(commands);
    register_ping(commands);
    register_route(commands);
    register_ship(commands);
    register_stats(commands);
    register_time(commands);
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

fn register_convert(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
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
    })
}

fn register_dialog(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
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

fn register_owoify(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
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

fn register_ship(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
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

fn register_time(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| {
        cmd.name("time")
            .description("Query the time of a city based on a city name or an address.")
            .create_option(|opt| {
                opt.name("city_name_or_address")
                    .description("A city name or an address of which to query time.")
                    .required(true)
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
