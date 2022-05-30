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
use std::sync::Arc;
use tokio::sync::Mutex;
use serenity::model::id::ChannelId;

struct ServerKey;

impl TypeMapKey for ServerKey {
    type Value = Arc<Mutex<ServerMap>>;
}

/* Contains information on all instances of Wordle that have been started. */
type ServerMap = HashMap<(ChannelId, UserId), Wordle>;

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let mut wordle_data = ctx.data.write().await;
    let wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordle map!");
    
    let wordle = Wordle::new();
    let mut string_response = String::from("Word: ");
    string_response.push_str(wordle.word.as_str());
    
    if let Err(why) = msg.channel_id.say(&ctx.http, &string_response).await {
        println!("Error sending message: {}", why);
    }
    
    /* Addding new wordle game for new client. */
    wordle_map
        .lock()
        .await
        .insert((msg.channel_id, msg.author.id), wordle);
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
        .type_map_insert::<ServerKey>(Arc::new(Mutex::new(ServerMap::new())))
        .await
        .expect("Couldn't create the new client!");
    if let Err(why) = client.start().await {
        println!("Client error: {}", why)
    }
}
