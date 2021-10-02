use crate::shared::services::judge_zero_service::{
    build_embed, create_eval_request, try_get_eval_result,
};
use crate::shared::structs::utility::judge_zero::JudgeZeroRequestResult;
use crate::shared::utility::{get_author_avatar, get_author_name};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::Error;

#[command]
#[description = "Evaluate Rust codes. This command will let share some Rust codes with Taiga/Kou and ask him for help in evaluating those codes."]
#[usage = "<Rust code, start and end with triple backticks."]
#[example = "```rust\nfn main() {\n    println!(\"Hello World!\");\n}```"]
#[bucket = "utility"]
pub async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(&ctx.http, "You need to provide Rust code first!")
            .await?;
        return Ok(());
    }

    let sent_msg = msg
        .reply(&ctx.http, "Alright! I got your code. Hold on a second...")
        .await?;

    let code_block = args
        .remains()
        .unwrap_or_default()
        .split('\n')
        .skip(1)
        .collect::<Vec<_>>();
    let actual_code: String = (&code_block[..code_block.len() - 1]).join("\n");

    let token = create_eval_request(actual_code).await.unwrap_or_default();
    let eval_result_handle = tokio::spawn(async move { try_get_eval_result(token).await });

    match eval_result_handle.await {
        Ok(result) => match result {
            Ok(result) => {
                handle_result(ctx, msg, sent_msg, result).await?;
            }
            Err(e) => {
                log::error!("Error when getting response from JudgeZero: {}", e);
            }
        },
        Err(e) => {
            log::error!("Failed to join eval result handle: {}", e);
        }
    }
    Ok(())
}

async fn handle_result(
    ctx: &Context,
    msg: &Message,
    mut sent_msg: Message,
    result: JudgeZeroRequestResult,
) -> CommandResult {
    match result {
        JudgeZeroRequestResult::Failed(res) => {
            let error_msg =
                "Sorry, but something went wrong and I can't seem to evaluate your code!\n"
                    .to_string();

            let error_msg = res
                .stderr
                .map(|s| format!("{}Error message: {}\n\n", &error_msg, s))
                .unwrap_or_else(|| error_msg);

            let error_msg = res
                .message
                .map(|s| format!("{}Extra message: {}\n\n", &error_msg, s))
                .unwrap_or_else(|| error_msg);

            sent_msg
                .edit(&ctx.http, |msg| msg.content(error_msg))
                .await?;
        }
        JudgeZeroRequestResult::InProgress => {
            sent_msg
                .edit(&ctx.http, |msg| {
                    msg.content("Sorry, I tried hard but can't seem to evaluate your code in time!")
                })
                .await?;
        }
        JudgeZeroRequestResult::Success(res) => {
            let member = msg.member(&ctx.http).await.ok();
            let author_name = get_author_name(&msg.author, &member);
            let author_avatar_url = get_author_avatar(&msg.author);
            let embed = build_embed(res, &author_name, &author_avatar_url);
            sent_msg
                .edit(&ctx.http, |msg| msg.content("").set_embed(embed))
                .await?;
        }
    }
    Ok(())
}
