use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::InterfaceService;

#[command]
#[aliases("credits", "bot")]
#[description = "Shows information about the bot."]
#[usage = ""]
#[example = ""]
#[bucket = "information"]
pub async fn about(context: &Context, msg: &Message) -> CommandResult {
    let data = context.data.read().await;
    let interface = data.get::<InterfaceService>().unwrap();
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(data);

    if !is_kou {
        let color_code = u32::from_str_radix("e81615", 16).unwrap();
        let description = "Taiga was inspired and loosely based on the amazing Yuuto, which was made and developed by the community, for the community. \n"
            .to_string();
        msg.channel_id.send_message(&context.http, |m| m.embed(|e| e
            .author(|a| a
                .name("Taiga from Camp Buddy")
                .icon_url("https://cdn.discordapp.com/emojis/593518771554091011.png")
                .url("https://blitsgames.com"))
            .color(color_code)
            .description(description +
                "It was also inspired by dunste123#0129's Hiro. \n" +
                "Join Yuuto's dev team and start developing on the [project website](http://iamdeja.github.io/yuuto-docs/). \n\n" +
                "Taiga version 3.0 was made and developed by: \n" +
                "**Tetsuki Syu#1250**, **Arch#0226**, **Kirito#9286** \n" +
                "Yuuto version 2.0 was made and developed by: \n" +
                "**Arch#0226**, **dunste123#0129**, **Tai Chi#4634**, **zsotroav#8941** \n" +
                "Taiga version 3.0 and Yuuto's Rust version ported by: \n**Tetsuki Syu#1250** \n" +
                "Japanese oracle co-translated with: \n**Kirito#9286** \n" +
                "Hangman, Quiz, and Tic-tac-toe minigames co-written with: \n**Kirito#9286** \n" +
                "Taiga reactions and feedback shared by: \n" +
                "**Kirito#9286**, **Kachiryoku#0387**, and countless Camp Buddy fans. \n" +
                "Taiga Bot is licensed under GNU GPLv3: https://www.gnu.org/licenses/gpl-3.0.en.html \n")
            .footer(|f| f.text("Taiga Bot: Release 3.0 | 2020-08-24 | For my most important person."))
            .thumbnail("https://cdn.discordapp.com/emojis/448579316171669545.png")))
            .await?;
    }
    else {
        let color_code = u32::from_str_radix("a4d0da", 16).unwrap();
        let description = "Kou was inspired by the anime/manga Jibaku Shōnen Hanako-kun (a.k.a. Toilet-Bound Hanako-kun). \n"
            .to_string();
        msg.channel_id.send_message(&context.http, |m| m.embed(|e| e
            .author(|a| a
                .name("Minamoto Kou from Jibaku Shōnen Hanako-kun")
                .icon_url("https://cdn.discordapp.com/emojis/705037054836473936.png")
                .url("https://www.tbs.co.jp/anime/hanakokun/"))
            .color(color_code)
            .description(description +
                "It was also inspired and based on [Taiga](https://www.github.com/deadshot465/TaigaBotCS) and inspired by the [Yuuto project](http://iamdeja.github.io/yuuto-docs/) \n" +
                "It was also a dedication and a tribute to my most important person. \n\n" +
                "Kou version 2.0 was made and developed by: \n" +
                "**Tetsuki Syu#1250**, **Arch#0226**, **Kirito#9286** \n" +
                "Taiga version 3.0 and Taiga's Rust version ported by: \n**Tetsuki Syu#1250** \n" +
                "Japanese oracle co-translated with: \n**Kirito#9286** \n" +
                "Hangman, Quiz, and Tic-tac-toe minigames co-written with: \n**Kirito#9286** \n" +
                "Kou Bot is licensed under GNU GPLv3: https://www.gnu.org/licenses/gpl-3.0.en.html \n")
            .footer(|f| f.text("Kou Bot: Release 2.0 | 2020-08-24 | For my most important person."))
            .thumbnail("https://cdn.discordapp.com/emojis/448579316171669545.png")))
            .await?;
    }

    Ok(())
}