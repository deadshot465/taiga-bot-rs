# TaigaBot

![Rust](https://github.com/deadshot465/taiga-bot-rs/workflows/Rust/badge.svg)

Taiga bot is a bot that aims to provide interactive experiences to the users in a private-owned Discord server for fans of Taiga, who is a character from a yaoi visual novel Camp Buddy by BLits.

Taiga bot is loosely based on and is a modified version of [yuuto-bot](https://github.com/Yuuto-Project/yuuto-bot), which is a community-driven project of Offical Camp Buddy Fan Server members, under GNU GPLv3 license. Yuuto bot's idea came from an increasing number of tech-oriented campers in the official fan server. While Yuuto is made by the community and for the community, the dialog choices and some design decisions are not meant for a specific character's fan server such as Taiga's fan server. Therefore, Taiga bot, while based on Yuuto bot and retains most features from Yuuto bot, aims to solve this problem and tailor to Taiga fan server's needs.

Over time, Taiga bot gradually got a couple of exclusive and new functionalities, and this is the third rewritten version of Taiga, coming after TypeScript and C#. Since the paradigm and design decisions of Rust is hugely different from those of C#, even though Taiga originally was inspired by Yuuto, the current base is very distinct now.

Taiga bot is also inspired by [hirobot](https://github.com/dunste123/hirobot) by dunste123 under the same license.

**Taiga bot is not the original version of Yuuto bot, but a rewritten version. Hence, if you are interested in the original version, please visit [yuuto-bot](https://github.com/Yuuto-Project/yuuto-bot) instead.**

*If you are interested in joining the project as a developer, please take time to check out Yuuto project's [website](https://iamdeja.github.io/yuuto-docs/).*

*See [hirobot](https://github.com/dunste123/hirobot) for the code base of Hiro bot.*

## Contents

- [Project Setup](#project-setup)
  - [Bot application](#bot-application)
  - [Why Rust](#why-rust)
  - [Setup steps](#setup-steps)
- [Differences between Taiga Bot and Yuuto Bot](#differences-between-taiga-bot-and-yuuto-bot)
- [Disclaimer](#disclaimer)

## Project Setup

Taiga bot is loosely based on Yuuto bot, which is originally written in JavaScript, having a dedicated repository [here](https://github.com/Yuuto-Project/yuuto-bot), and now in the process of being ported to Kotlin. You can find the Kotlin version of Yuuto (Kyuuto) [here](https://github.com/Yuuto-Project/kyuuto/). However, Taiga bot is ported and rewritten in the stable version of Rust.

### Bot application

The bot is a port and a rewritten version of Yuuto bot in Rust. As such, it is run on [the stable version of Rust](https://www.rust-lang.org/) and uses [the async branch of Serenity](https://github.com/Lakelezz/serenity/tree/await). **Please be advised that it's not written with the nightly build of Rust. Also, since Rust is cross-platform, there shouldn't be any problem in compiling and executing Taiga in any major operating system.** Setup steps are described later.

### Why Rust

JavaScript, while being a de facto language choice when it comes to web development, is a weak-typed language. This makes it more challenging to track each variable and return value's types. As a result, it's not uncommon for the developer to manually track variable's types or assume the available methods and properties of a variable. Also, it's also more challenging for IDEs to provide static type checking and IntelliSense. Therefore, in order to ease the burden when rewriting parts of Yuuto bot's codes, TypeScript was chosen and actively used in as many circumstances as possible. You can read more about TypeScript [here](https://www.typescriptlang.org/).

However, as the developers of Yuuto started seeking more robust languages than JavaScript, Kotlin then became the primary choice of the future developments of Yuuto. Given the fact that future developments of Yuuto might be migrated to using Kotlin, in order to adopt incoming changes more easily, Taiga bot was again rewritten with .NET Core 3.1 and C# 8.0.

C# is a robust language and Discord.Net is a powerful framework for writing Discord bots, so there's really no strong reasons to rewrite Taiga again. Therefore, initially the Rust version of Taiga was meant to be my first project in Rust and a simple practice for me to get familiar with Rust. Nonetheless, due to the nature of Rust being a high performance and system programming language, plus the async, non-blocking functionality powered by [Tokio](https://tokio.rs/), the performance of the resulting rewritten version of Taiga is inherently better than C# version. Also, its emphasis on memory safety, strict borrow checker and lifetime, and the lack of `null` make the final result even safer and predictable.

### Setup steps

This repo doesn't include compiled files, which usually are stored under the `target` folder of the project root directory. Therefore, whether you are interested in hosting Taiga bot on your own or are just interested in the code, there are some required steps before you can compile the code.

1. [Install Rust](https://www.rust-lang.org/) with methods that apply to your operating system. Using [JetBrains CLion](https://www.jetbrains.com/clion/) or [JetBrains IntelliJ IDEA](https://www.jetbrains.com/idea/) with [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust?_ga=2.189013035.1679919971.1594635449-1551243450.1584093519) is strongly recommended, as Taiga is developed with these IDEs. If you prefer installing Rust via command line, assuming you're using Ubuntu, you can use the following commands to install Rust:

   ```bash
   curl https://sh.rustup.rs -sSf | sh
   ```
   
2. Clone this repository with:

   ```bash
   git clone https://github.com/deadshot465/taiga-bot-rs.git
   ```

3. Assuming you're using IntelliJ IDEA or CLion, open up the project directory, the IDE should take care and download required Crate packages for you. Alternatively, use the following commands to either restore Crate packages, build the project, or run the project via command line. **Please be advised that the project will not run when the `.env` file is not present.**

   ```bash
   cd <path-to-the-root-directory-of-the-repo>
   
   # This will only build the project but not run it
   # Omitting --release will result in building with Debug configuration
   cargo build --release
   
   # This will build and run immediately
   cargo run --release
   ```
   
4. Provided that you have created your own application on Discord, you can manually create a file named `.env` in the same location as the compiled executable named `TaigaBotCS.exe` (Windows) or the respective files on other platforms, as the program will read required tokens and environment variables from this file. An unmodified version of Taiga bot expects the following variables/tokens from `.env`:

   ```
   TOKEN = <Your Discord application token here>
   PREFIX = <The bot's command prefix>
   GENCHN = <The primary general channel's id>
   BOTCHN = <Dedicated bot commands channel's id>
   BOTMODCHN = <Dedicated bot commands channel's id that is only accessible by mods>
   TESTGENCHN = <Another personal test server's general channel id>
   TESTCHN = <Another personal test server's test channel id>
   VENTCHN = <Venting center channel id, as some channels are not meant for bot's random response>
   KOUGENCHN = <The primary general channel's id in Kou stan server>
   ECC_GENCHN = <Only for personal use. You can remove the need for it by editing the source file>
   UNSPLASH_TOKEN = <This bot uses Unsplash's API to acquire certain images. This is the token of your Unsplash application>
   BOT_ID = <Your Discord bot's ID. This is different from the token>
   MENTION_REACTION_CHANCE = <When Taiga is mentioned/pinged, the chance of he responding to the message>
   REACTION_CHANCE = <The probability of Taiga reacting to messages related to certain characters using emote/emojis>
   RDM_REPLY_CHANCE = <The probability of Taiga replying to messages related to certain characters>
   RDM_REPLY_USER_CHANCE = <The probability of Taiga replying to users (a.k.a. personalized replies)>
   SPECIALIZED_CHANCE = <The probability of Taiga replying to messages related to certain characters using specialized messages>
   TECHCHN = <The channel id of the tech club channel in Taiga stan server>
   LOGIN_NAME = <Required user name to authenticate with the API endpoint for dialog/say/comic>
   LOGIN_PASS = <Required user password to authenticate with the API endpoint for dialog/say/comic>
   ```

   **All placeholder texts should be replaced with your own content, without quotation marks (`"` and `'`) and greater than/less than (`<` and `>`) symbols.**

5. Once you set up, compile the program to run the bot.


## Differences between Taiga Bot and Yuuto Bot

The main difference is, without a doubt, that Taiga bot is written in Rust, while Yuuto bot is written in JavaScript and later in Kotlin. Since Rust is profoundly different from OOP languages, some detailed descriptions include, but not limited to, the following:

1. There are no classes and inheritance in Rust, since Rust is **not** an OOP language. Therefore, all commands are function, macro and attribute based.
3. `CalculcateScore` method in `ship` command returns a `(u8, Cow<'a, str>)`.
4. All parameters of methods are typed, as is required in Rust.
5. Taiga bot uses the async branch of Serenity, while Yuuto bot uses a customized version of Discord.js and later JDA.
6. `cvt` command directly queries a `HashMap<String, HashMap<String, f64>>` and doesn't convert to Kelvin first when calculating temperatures.
7. Commands, aliases and cooldowns are not properties of the client; instead, they are directly denoted on attributes and groups.
8. Certain dialogs and reactions are changed to add more flavors to Taiga.
9. Several commands are added and more commands will be implemented as well as the time passes.
10. `about` command shows a modified version of information to add disclaimers and other supporters during the porting and rewriting of Yuuto bot's code.
11. Most services are implemented using unsafe mutable static variables now, since in safe Rust codes, mutable static variables are disallowed.
12. As there is no `Promise` in Rust, `async`, `await` and `impl Future` are heavily used.

## Disclaimer

Taiga bot will not be possible without the reference of Yuuto bot. All credit for Yuuto bot's existing functionalities goes to the developers of Yuuto bot and the community. Please refer to the `about` command for more details.

- [Yuuto Project](https://iamdeja.github.io/yuuto-docs/)
- [Yuuto-bot Repository](https://github.com/Yuuto-Project/yuuto-bot)
- [Kyuuto Project](https://kyuuto.io/)
- [Kyuuto Repository](https://github.com/Yuuto-Project/kyuuto)
- [hirobot](https://github.com/dunste123/hirobot) (by dunste123)
- [Blits Games](https://www.blitsgames.com/)
- [Official Camp Buddy Fan Server](https://discord.gg/campbuddy) (on Discord)