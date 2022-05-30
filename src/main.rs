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
use serenity::framework::standard::Args;
use crate::wordle::GUESSES;
use crate::words::Words;

struct ServerKey;

impl TypeMapKey for ServerKey {
    type Value = Arc<Mutex<ServerMap>>;
}

/* Contains information on all instances of Wordle that have been started. */
struct ServerMap {
    games: HashMap<(ChannelId, UserId), Wordle>,
    words: Words,
}

impl ServerMap {
    pub async fn new() -> ServerMap {
        ServerMap {
            games: HashMap::new(),
            words: Words::new().await,
        }
    }
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) =
    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Hello, I'm a Wordle Bot")
                .description("Type `!start` to start the game.")
        })
    }).await {
        println!("Error sending the help message: {}", why);
    }
    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let mut wordle_data = ctx.data.write().await;
    let wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordles map!");

    let random_word = wordle_map.lock().await.words.generate_word().word();
    let wordle = Wordle::new(random_word.clone());
    let mut string_response = String::from("Word: ");
    string_response.push_str(random_word.as_str());

    if let Err(why) = msg.channel_id.say(&ctx.http, &string_response).await {
        println!("Error sending message: {}", why);
    }
    wordle_map
        .lock()
        .await
        .games
        .insert((msg.channel_id, msg.author.id), wordle);
    Ok(())
}

#[command]
async fn guess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guess = args.single_quoted::<String>()?.to_uppercase();

    let mut wordle_data = ctx.data.write().await;
    let mut wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordles map!").lock().await;
    let mut wordle = wordle_map.games.get_mut(&(msg.channel_id, msg.author.id));

    let mut string_response = String::from("");
    if wordle.is_none() {
        string_response.push_str("To play the game write !start");
    } else {
        let mut wordle = wordle.unwrap();
        wordle.guesses += 1;
        if guess.eq(&wordle.word) {
            string_response.push_str("You won! 🎉");
            wordle_map.games.remove(&(msg.channel_id, msg.author.id));
        } else if wordle.guesses == GUESSES {
            string_response.push_str("You ran out of guesses!\nThe word was: ");
            string_response.push_str(wordle.word.as_str());
            wordle_map.games.remove(&(msg.channel_id, msg.author.id));
        } else {
            string_response.push_str("Guess again!");
        }
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &string_response).await {
        println!("Error sending message: {}", why);
    }
    Ok(())
}

/* Declaration of a set of available commands. */
#[group("public")]
#[commands(start, guess, help)]
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
        .type_map_insert::<ServerKey>(Arc::new(Mutex::new(ServerMap::new().await)))
        .await
        .expect("Couldn't create the new client!");
    if let Err(why) = client.start().await {
        println!("Client error: {}", why)
    }
}
