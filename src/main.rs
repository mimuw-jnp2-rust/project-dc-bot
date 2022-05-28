mod config;
use config::Config;
use serenity::{
    prelude::*,
    model::prelude::*,
    client::ClientBuilder,
    framework::standard::{
        CommandResult, macros::command, macros::group, StandardFramework,
    }
};

#[command]
async fn start(ctx: & Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "jazda").await {
        println!("Error sending message: {}", why);
    }
    return Ok(());
}

/* Declaration of a set of available commands. */
#[group]
#[commands(start)]
struct Public;

#[tokio::main]
async fn main() {
    let _ = Config::new().save();
    let config = Config::load().unwrap();
    let mut client = ClientBuilder::new(config.token(), GatewayIntents::default())
        .framework(StandardFramework::new()
            .configure(|c|
                c.prefix(config.prefix()))
            .group(&PUBLIC_GROUP))
        .await
        .expect("Couldn't create the new client!");
    if let Err(why) = client.start().await {
        println!("Client error: {}", why)
    }

}