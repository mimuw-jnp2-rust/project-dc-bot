mod config;
mod wordle;
mod words;

use std::collections::HashMap;
use config::Config;
use wordle::Wordle;
use serenity::{
    client::ClientBuilder,
    framework::standard::{macros::command, macros::group, CommandResult, StandardFramework},
    model::prelude::*,
    model::id::*,
    prelude::*,
};

/* Struct containing information on all instances of Wordle that have been started. */
struct Server{
    games : HashMap<UserId, Wordle>
}

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "jazda").await {
        println!("Error sending message: {}", why);
    }
    Ok(())
}

/* Declaration of a set of available commands. */
#[group("public")]
#[commands(start)]
struct Public;

#[tokio::main]
async fn main() {
    let _ = Config::new().save();
    let config = Config::load().unwrap();
    let mut client = ClientBuilder::new(config.token(), GatewayIntents::GUILD_MESSAGES.union(GatewayIntents::MESSAGE_CONTENT))
        .framework(
            StandardFramework::new()
                .configure(|c| c.with_whitespace(true).prefix(config.prefix()))
                .group(&PUBLIC_GROUP),
        )
        .await
        .expect("Couldn't create the new client!");
    if let Err(why) = client.start().await {
        println!("Client error: {}", why)
    }
}