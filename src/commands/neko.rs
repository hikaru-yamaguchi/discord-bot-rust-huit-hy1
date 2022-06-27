use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "meow"]
async fn neko(ctx: &Context, msg: &Message) -> CommandResult {
    // throw a message on channel (channel_id) msg.channel_id.say
    msg.channel_id
        .say(&ctx.http, format!("{} meow", msg.author.mention()))
        .await?;
    // CommandResult(Result)
    Ok(())
}