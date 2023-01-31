use docker_api::Docker;
use dotenv::dotenv;
use ds_code::run_user_code;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler, GatewayIntents};
use serenity::{async_trait, Client};
use std::env;
use std::error::Error;

#[group]
#[commands(run)]
struct GeneralCommands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERALCOMMANDS_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN needs to be set.");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}

#[command]
async fn run(ctx: &Context, msg: &Message) -> CommandResult {
    let msg_content = msg.content.clone();
    let splitted_msg: Vec<_> = msg_content.split("```").collect();

    let user_code_block = splitted_msg.get(1);

    if user_code_block.is_none() {
        msg.reply(ctx, "Can not find code block.").await?;
        return Ok(());
    }

    let lines: Vec<_> = user_code_block.unwrap().lines().collect();
    let code_lang = lines.get(0);
    let user_code = &lines[1..].join("\n");

    // Running docker
    let docker = Docker::new(env::var("DOCKER_HOST")?)?;

    let code_output = run_user_code(&docker, user_code, code_lang.unwrap_or(&"UNKNOWN")).await;

    if code_output.is_err() {
        println!("{:?}", code_output);
    }

    msg.reply(
        ctx,
        format!(
            "```\n{}\n```",
            if code_output.is_ok() {
                let code_output = code_output.unwrap();

                if code_output.len() > 1900 {
                    format!(
                        "{}\n...output is too big (printed first 1900 chars)",
                        &code_output[..1900]
                    )
                } else {
                    String::from(&code_output)
                }
            } else {
                format!("Failed to process. {:?}", code_output)
            }
        ),
    )
    .await?;

    Ok(())
}
