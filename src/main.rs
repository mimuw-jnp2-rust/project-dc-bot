mod config;
mod wordle;
mod words;

use crate::wordle::{DEFAULT_SIZE, GUESSES};
use crate::words::Words;
use config::Config;
use serenity::{
    client::ClientBuilder,
    framework::standard::{macros::command, macros::group, Args, CommandResult, StandardFramework},
    model::id::*,
    model::prelude::*,
    prelude::*,
};
use std::collections::HashMap;
use std::sync::Arc;
use string_builder::Builder;
use tokio::sync::Mutex;
use wordle::Wordle;

pub static HELLO_MSG: &str = "Hello, I'm a Wordle Bot";

/* Structure to share data across server. */
struct ServerKey;

impl TypeMapKey for ServerKey {
    type Value = Arc<Mutex<ServerMap>>;
}

/* Contains information on all instances of Wordle that have been started and
all available words to guess. */
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

async fn send_embed_message(ctx: &Context, msg: &Message, message: &str) {
    if let Err(why) = msg
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|e| e.title(HELLO_MSG).description(message))
        })
        .await
    {
        println!("Error sending the help message: {}", why);
    }
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed_message(ctx, msg, &format!("Type `!start` to start the game.\n**Rules:**\nYou have {} tries to guess a {}-letter word.\n\
    To guess type `!guess [Your guess]`.\nAfter each guess the color of the letters will change to show how close your guess was to the word.\n\
    If the letter is **green**, it is in the word and in the correct spot.\nIf the letter is **yellow**, it is in the word but in the wrong spot.\n\
    If the letter is **red**, it is not in the word in any spot.", GUESSES, DEFAULT_SIZE)).await;
    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    /* Gets shared data across whole server. */
    let mut wordle_data = ctx.data.write().await;
    let wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordle map!");

    let wordle = Wordle::new(wordle_map.lock().await.words.generate_word().word.clone());

    send_embed_message(
        ctx,
        msg,
        "Game started! Take a guess using `!guess [Your guess]`.",
    )
    .await;
    wordle_map
        .lock()
        .await
        .games
        .insert((msg.channel_id, msg.author.id), wordle);
    Ok(())
}

#[command]
async fn guess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut wordle_data = ctx.data.write().await;
    let mut wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordle map!")
        .lock()
        .await;
    let words_vector: Vec<String> = wordle_map
        .words
        .words
        .iter()
        .map(|word| word.word.clone())
        .collect();

    /* Word comparison is case insensitive. */
    let guess = args.single_quoted::<String>()?.to_uppercase();
    let mut string_response = Builder::default();

    if guess.len() != DEFAULT_SIZE || !guess.chars().all(char::is_alphabetic) {
        string_response.append("Guess word must contain 5 letters without numbers");
    } else if !words_vector.contains(&guess) {
        string_response.append("Guess word is not in word list");
    } else {
        let wordle = wordle_map.games.get_mut(&(msg.channel_id, msg.author.id));

        if wordle.is_none() {
            string_response.append("To play the game type !start");
        } else {
            let mut wordle = wordle.unwrap();
            wordle.guesses += 1;

            /* Check if the guess was correct or if the person ran out of guesses.
            If not, add guess to the list and display all guesses. */
            if guess.eq(&wordle.word) {
                string_response.append("You won! 🎉");
                wordle_map.games.remove(&(msg.channel_id, msg.author.id));
            } else if wordle.guesses == GUESSES {
                string_response.append("You ran out of guesses!\nThe word was: ");
                string_response.append(wordle.word.as_str());
                wordle_map.games.remove(&(msg.channel_id, msg.author.id));
            } else {
                wordle.add_fields(guess);
                wordle.display_game(&mut string_response);
                string_response.append("Guess again!");
            }
        }
    }

    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, &string_response.string().unwrap())
        .await
    {
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
    let mut client = ClientBuilder::new(
        config.token(),
        GatewayIntents::GUILD_MESSAGES.union(GatewayIntents::MESSAGE_CONTENT),
    )
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
