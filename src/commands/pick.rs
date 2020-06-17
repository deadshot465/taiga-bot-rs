use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;

const COMMAND_LENGTH: usize = 8;

#[command]
pub async fn pick(context: &Context, msg: &Message) -> CommandResult {
    let raw_options = &msg.content[COMMAND_LENGTH..];
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

    if options.len() <= 0 {
        msg.channel_id.say(&context.http, "This command requires at least two options: `pick <option1> | <option2>...`")
            .await?;
        return Ok(());
    }

    let raw_times: Vec<&str> = options[0].split(' ').collect();
    let arg = raw_times[0].trim().to_lowercase();
    if arg.ends_with("times") {
        let index = arg.find('t').unwrap();
        let start = arg.find('s').unwrap();
        options[0] = &options[0][(start + 2)..];
        let times = arg[..index].parse::<i64>()?;
        let mut options_map = HashMap::new();
        for option in options.iter() {
            options_map.insert(*option, 0 as i64);
        }
        {
            let mut rng = thread_rng();
            for _ in 0..times {
                *options_map.entry(options[rng.gen_range(0, options.len())])
                    .or_insert(0) += 1;
            }
        }
        let mut sorted_list: Vec<_> = options_map.into_iter().collect();
        sorted_list.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result_message = format!("<:TaigaSmug:702210822310723614> I pick **{}**!\n", &sorted_list[0].0);
        for pair in sorted_list.iter() {
            result_message += format!("{} - {} times\n", (*pair).0, (*pair).1).as_str();
        }
        msg.channel_id.say(&context.http, result_message.as_str())
            .await?;
    }
    else {
        let result = options[thread_rng().gen_range(0, options.len())];
        msg.channel_id.say(&context.http, format!("<:TaigaSmug:702210822310723614> I pick **{}**!", result))
            .await?;
    }

    for x in options.iter() {
        println!("{}", x);
    }

    Ok(())
}