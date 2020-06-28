use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;
use crate::shared::CommandStrings;
use crate::INTERFACE_SERVICE;

const COMMAND_LENGTH: usize = 8;

#[command]
#[aliases("choose")]
#[description = "Pick from several options."]
#[usage = "<option1> | <option2> | <option3>..."]
#[example = "A | B | C"]
#[bucket = "utilities"]
pub async fn pick(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.pick;
    }

    if args.is_empty() || args.len() == 0 {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    let first_arg = args.single::<String>().unwrap();
    let mut is_multiple = false;
    if first_arg.to_lowercase().ends_with("times") {
        if args.is_empty() || args.len() == 0 {
            let error_msg = interface_string.errors["length_too_short"].as_str();
            msg.channel_id.say(&context.http, error_msg).await?;
            return Ok(());
        }
        else {
            is_multiple = true;
        }
    }

    let raw_options = args.remains().unwrap();
    let options_unsanitized: Vec<&str> = raw_options.split('|')
        .collect();
    let mut options: Vec<&str> = options_unsanitized
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
    if !is_multiple {
        options.push(first_arg.as_str());
    }

    if options.len() <= 0 {
        let message = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, message)
            .await?;
        return Ok(());
    }

    if is_multiple {
        let index = first_arg.find('t').unwrap();
        let times = &first_arg[..index].parse::<u32>();
        let mut options_map = HashMap::new();
        for option in options.iter() {
            options_map.insert(*option, 0 as u32);
        }
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
        let mut sorted_list: Vec<_> = options_map.into_iter().collect();
        sorted_list.sort_by(|a, b| b.1.cmp(&a.1));
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