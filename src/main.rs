mod commands;

use std::{collections::HashSet, fs::File, io::BufReader, usize};

use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler, GatewayIntents};

use serde::{Deserialize, Serialize};
use serde_json::Result;

/// Handler Struct. Implements events to fetch.
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// handles when bot starts
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[command]
async fn neko(ctx: &Context, msg: &Message) -> CommandResult {
    // throw a message on channel (channel_id) msg.channel_id.say
    println!("hello neko!");
    msg.channel_id.say(&ctx.http, "meow").await?;
    // CommandResult(Result)
    Ok(())
}

#[group("neko")]
#[commands(neko)]
struct Neko;

#[help]
#[individual_command_tip = "This is `help` command."] // help explanation
#[strikethrough_commands_tip_in_guild = ""] // unavailable commands
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    println!("my_help was called!");
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
        .await
        .expect("not called");
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

fn get_token(file_name: &str) -> Result<String> {
    let file = File::open(file_name).expect("Token file not found");
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}

#[tokio::main]
async fn main() {
    let token = get_token("../config.json").expect("Err could not find token");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~".to_string()))
        .group(&NEKO_GROUP);

    let mut client = Client::builder(&token, GatewayIntents::default())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
