use crate::InterfaceService;
use rand::prelude::*;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::collections::HashMap;
use std::sync::Arc;

#[command]
#[aliases("choose")]
#[description = "Pick from several options."]
#[usage = "<option1> | <option2> | <option3>..."]
#[example = "A | B | C"]
#[bucket = "utilities"]
pub async fn pick(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = context.data.read().await;
    let interface = data
        .get::<InterfaceService>()
        .expect("Failed to get interface service.");
    let _interface = Arc::clone(interface);
    drop(data);
    let interface_lock = _interface.read().await;
    let interface = interface_lock
        .interface_strings
        .as_ref()
        .expect("Failed to get interface strings.");
    let interface_string = &interface.pick;

    // If there are no arguments at all, abort.
    if args.is_empty() {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    // Get the first argument in the list.
    let first_arg = args
        .single::<String>()
        .expect("Failed to get first argument.");
    let mut is_multiple = false;
    let mut first_arg_piped = false;
    let mut raw_times = String::new();
    // Test if the first argument has pipe signs.
    // If there is, split them and add to available options.
    let mut options: Vec<String> = vec![];
    // E.g. k!pick 5000times|A|B|C
    if first_arg.contains('|') {
        let split_first_args = first_arg.split('|').collect::<Vec<&str>>();
        let mut split_first_args = split_first_args
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if split_first_args[0].ends_with("times") {
            // Store the raw times string into another variable.
            raw_times = split_first_args.remove(0);
            is_multiple = true;
        }
        if !split_first_args.is_empty() {
            options.append(&mut split_first_args);
            first_arg_piped = true;
        }
    } else {
        // E.g. k!pick 5000times A|B|C
        if first_arg.ends_with("times") {
            raw_times = first_arg;
            is_multiple = true;
        }
    }

    // If it's multiple picks but there are no remaining arguments, and the option list is empty, abort.
    if args.is_empty() && (is_multiple && options.is_empty()) {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    // Get remaining options.
    if is_multiple {
        if let Some(s) = args.remains() {
            let options_unsanitized: Vec<&str> = s.split('|').collect();
            let mut options_unsanitized = options_unsanitized
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            options.append(&mut options_unsanitized);
            if first_arg_piped {
                let mut first_option = options.remove(0);
                first_option += " ";
                first_option += options[0].as_str();
                options[0] = first_option;
            }
        }
    } else {
        let options_unsanitized = args.message().split('|').collect::<Vec<&str>>();
        let mut options_unsanitized = options_unsanitized
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        options.append(&mut options_unsanitized);
    }
    // Sanitize the options.
    let options: Vec<String> = options
        .into_iter()
        .filter_map(|s| {
            let _s = s.trim();
            if !_s.is_empty() {
                Some(_s.to_string())
            } else {
                None
            }
        })
        .collect();

    // Check if there are actually options.
    if options.is_empty() {
        let message = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, message).await?;
        return Ok(());
    }

    if is_multiple {
        // Parse the times.
        let index = raw_times
            .find('t')
            .expect("Failed to find raw times in the input string.");
        let times = &raw_times[..index].parse::<u32>();
        // Map all options to a map.
        let mut options_map = HashMap::new();
        for option in options.iter() {
            options_map.insert(option.as_str(), 0 as u32);
        }
        // Do the calculation.
        {
            if times.is_err() {
                let error_msg = interface_string.errors["times_too_big"].as_str();
                msg.channel_id.say(&context.http, error_msg).await?;
                return Ok(());
            } else {
                let mut rng = thread_rng();
                for _ in 0..*times.as_ref().expect("Failed to parse u32 from string.") {
                    *options_map
                        .entry(options[rng.gen_range(0..options.len())].as_str())
                        .or_insert(0) += 1_u32;
                }
            }
        }
        // Map the option map into a vector of tuples, and perform sorting.
        let mut sorted_list: Vec<_> = options_map.into_iter().collect();
        sorted_list.sort_by(|a, b| b.1.cmp(&a.1));
        // Build the message and send.
        let mut message = interface_string
            .result
            .as_str()
            .replace("{option}", &sorted_list[0].0);
        message += "\n";
        for pair in sorted_list.iter() {
            message += format!("{} - {} times\n", (*pair).0, (*pair).1).as_str();
        }
        msg.channel_id.say(&context.http, message.as_str()).await?;
    } else {
        let result = options[thread_rng().gen_range(0..options.len())].as_str();
        let message = interface_string.result.as_str().replace("{option}", result);
        msg.channel_id.say(&context.http, message).await?;
    }
    drop(interface_lock);
    Ok(())
}
