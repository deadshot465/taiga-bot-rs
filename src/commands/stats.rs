use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::CommandStrings;
use crate::INTERFACE_SERVICE;

const VALID_COMMANDS: [&'static str; 2] = [
    "route", "valentine"
];

#[command]
#[description = "This command will show your records with several commands."]
#[usage = "<command>"]
#[example = "valentine"]
#[only_in("guilds")]
#[bucket = "information"]
pub async fn stats(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.stats;
    }
    let user_id = msg.author.id.0.to_string();
}