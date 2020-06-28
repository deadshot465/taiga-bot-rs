use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;
use crate::shared::CommandStrings;
use crate::INTERFACE_SERVICE;

#[command]
#[aliases("choose")]
#[description = "Pick from several options."]
#[usage = "<option1> | <option2> | <option3>..."]
#[example = "A | B | C"]
#[bucket = "utilities"]
pub async fn pick(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get interface strings.
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.pick;
    }

    // If there are no arguments at all, abort.
    if args.is_empty() || args.len() == 0 {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    // Get the first argument in the list.
    let first_arg = args.single::<String>().unwrap();
    let mut is_multiple = false;
    let mut raw_times = String::new();
    // Test if the first argument has pipe signs.
    // If there is, split them and add to available options.
    let mut options: Vec<&str> = vec![];
    // E.g. k!pick 5000times|A|B|C
    if first_arg.contains('|') {
        let mut split_first_args = first_arg.split('|').collect::<Vec<&str>>();
        if split_first_args[0].ends_with("times") {
            // Store the raw times string into another variable.
            raw_times = split_first_args.remove(0).to_string();
            is_multiple = true;
        }
        options.append(&mut split_first_args);
    }
    else {
        // E.g. k!pick 5000times A|B|C
        if first_arg.ends_with("times") {
            raw_times = first_arg;
            is_multiple = true;
        }
        else {
            // E.g. k!pick A
            options.push(first_arg.as_str());
        }
    }

    // If it's multiple picks but there are no remaining arguments, and the option list is empty, abort.
    if (args.len() == 0 || args.is_empty()) && (is_multiple && options.len() == 0) {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    // Get remaining options.
    if let Some(s) = args.remains() {
        let mut options_unsanitized: Vec<&str> = s.split('|')
            .collect();
        options.append(&mut options_unsanitized);
    }
    // Sanitize the options.
    let options: Vec<&str> = options
        .into_iter()
        .filter_map(|s| {
            let _s = s.trim();
            if _s.len() > 0 {
                Some(_s)
            }
            else {
                None
            }
        })
        .collect();

    // Check if there are actually options.
    if options.len() <= 0 {
        let message = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, message)
            .await?;
        return Ok(());
    }

    if is_multiple {
        // Parse the times.
        let index = raw_times.find('t').unwrap();
        let times = &raw_times[..index].parse::<u32>();
        // Map all options to a map.
        let mut options_map = HashMap::new();
        for option in options.iter() {
            options_map.insert(*option, 0 as u32);
        }
        // Do the calculation.
        {
            if let Err(_) = times {
                let error_msg = interface_string.errors["times_too_big"].as_str();
                msg.channel_id.say(&context.http, error_msg).await?;
                return Ok(());
            }
            else {
                let mut rng = thread_rng();
                for _ in 0..*times.as_ref().unwrap() {
                    *options_map.entry(options[rng.gen_range(0, options.len())])
                        .or_insert(0) += 1_u32;
                }
            }
        }
        // Map the option map into a vector of tuples, and perform sorting.
        let mut sorted_list: Vec<_> = options_map.into_iter().collect();
        sorted_list.sort_by(|a, b| b.1.cmp(&a.1));
        // Build the message and send.
        let mut message = interface_string.result.as_str().replace("{option}", &sorted_list[0].0);
        message += "\n";
        for pair in sorted_list.iter() {
            message += format!("{} - {} times\n", (*pair).0, (*pair).1).as_str();
        }
        msg.channel_id.say(&context.http, message.as_str())
            .await?;
    }
    else {
        let result = options[thread_rng().gen_range(0, options.len())];
        let message = interface_string.result.as_str().replace("{option}", result);
        msg.channel_id.say(&context.http, message)
            .await?;
    }

    Ok(())
}