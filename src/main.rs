mod config;
mod wordle;
mod words;

use crate::wordle::{DEFAULT_SIZE, GUESSES};
use crate::words::Words;
use config::Config;
use const_format::formatcp;
use serenity::{
    client::ClientBuilder,
    framework::standard::{macros::command, macros::group, Args, CommandResult, StandardFramework},
    model::id::*,
    model::prelude::*,
    prelude::*,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use std::vec::Vec;
use string_builder::Builder;
use tokio::sync::{Mutex, MutexGuard};
use wordle::Wordle;

/* Messages send by bot. */
pub const HELLO_MSG: &str = "Hello, I'm a Wordle Bot";
pub const HELP_MSG: &str = formatcp!("Type `!start` to start the game.\n**Rules:**\nYou have {} tries to guess a {}-letter word in 5 minutes.\n\
    To guess type `!guess [Your guess]`.\nAfter each guess the color of the letters will change to show how close your guess was to the word.\n\
    If the letter is **green**, it is in the word and in the correct spot.\nIf the letter is **yellow**, it is in the word but in the wrong spot.\n\
    If the letter is **red**, it is not in the word in any spot.\n\n Type `!start <number_of_players>` to start a game with friends.\n\
    **Additional rules for groups:**\nYou have 5 minutes to gather a specified number of players.\nTo join a group type `!join`.\n\
    A group can only play if there are no solo games and if there are no other groups playing.", GUESSES, DEFAULT_SIZE);
pub const GROUP_PLAYING_MSG: &str = "A group is playing, wait for the game to finish!";
pub const GAME_STARTED_MSG: &str = "Game started! Take a guess using `!guess [Your guess]`.";
pub const WRONG_PLAYERS_NUMBER_MSG: &str = "If you want to play alone type `!start`! \
     If you want to play in a group, you need at least two players!";
pub const SOLO_PLAYING_MSG: &str = "Someone is playing, wait for the game(s) to finish!";
pub const WAIT_FOR_PLAYERS_MSG: &str = "Wait for other players to start the game! \
     To join a game type `!join`.";
pub const ALREADY_JOINED_MSG: &str = "You already joined a group!";
pub const WRONG_CHANNEL_MSG: &str = "If you want to join your friends type `!join` \
     on a channel where the game was initiated!";
pub const START_GROUP_MSG: &str = "To start playing with friends type `!start <number_of_players>`";
pub const GUESS_WRONG_CHANNEL_MSG: &str = "Type your guess on a channel where the game started!";
pub const NOT_IN_GROUP_MSG: &str = "You can't guess the word as you are not in a group!";
pub const INCORRECT_GUESS_MSG: &str = "Guess word must contain 5 letters without numbers";
pub const NOT_IN_LIST_MSG: &str = "Guess word is not in word list";
pub const START_PLAYING_MSG: &str = "If you want to play alone type `!start`! \
     To start playing with friends, type `!start <number_of_player>`!";
pub const WON_MSG: &str = "You won! 🎉";
pub const TOO_MANY_GUESSES_MSG: &str = "You ran out of guesses!\nThe word was: ";
pub const YOUR_GUESSES_MSG: &str = " your guesses: \n";
pub const GUESS_AGAIN: &str = "Guess again!";

/* Every solo player has 5 minutes to complete game.
Group players have 5 minutes to join a game and another 5 minutes to play. */
pub const GAME_TIME: u64 = 5 * 60;

/* Structure to share data across server. */
struct ServerKey;

impl TypeMapKey for ServerKey {
    type Value = Arc<Mutex<ServerMap>>;
}

/* Contains information on all instances of Wordle that have been started,
max people playing, vector of people that joined group play and all available words to guess. */
struct ServerMap {
    games: HashMap<(ChannelId, UserId), (Wordle, SystemTime)>,
    /* Takes value: one if there is at least one solo play or
    number of players in a group if there is a group play. */
    max_people_playing: usize,
    joined_people: Vec<UserId>,
    words: Words,
}

impl ServerMap {
    pub async fn new() -> ServerMap {
        ServerMap {
            games: HashMap::new(),
            max_people_playing: 1,
            joined_people: Vec::new(),
            words: Words::new().await,
        }
    }
}

async fn send_embed_message(ctx: &Context, msg: &Message, message: &str) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|e| e.title(HELLO_MSG).description(message))
        })
        .await
    {
        println!("Error sending the help message: {}", why);
    }
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    send_embed_message(ctx, msg, HELP_MSG).await
}

/* Removes all games that took longer than 5 minutes to play/gather enough players. */
fn check_ended_games(wordle_map: &mut MutexGuard<'_, ServerMap>) {
    wordle_map
        .games
        .retain(|_, time| time.1.elapsed().expect("Failed to get time!").as_secs() < GAME_TIME);
}

async fn add_new_wordle(msg: &Message, wordle_map: &mut Arc<Mutex<ServerMap>>) {
    let wordle = Wordle::new(wordle_map.lock().await.words.generate_word().word.clone());
    wordle_map
        .lock()
        .await
        .games
        .insert((msg.channel_id, msg.author.id), (wordle, SystemTime::now()));
}

#[command]
async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    /* Gets shared data across whole server. */
    let mut wordle_data = ctx.data.write().await;
    let wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordle map!");

    check_ended_games(&mut wordle_map.lock().await);

    /* No one can start a game if a group is playing/gathering players. */
    if wordle_map.lock().await.max_people_playing > 1 {
        return send_embed_message(ctx, msg, GROUP_PLAYING_MSG).await;
    }

    /* Starting game for solo player. */
    if args.is_empty() {
        add_new_wordle(msg, wordle_map).await;
        return send_embed_message(ctx, msg, GAME_STARTED_MSG).await;
    }

    let number_of_players: usize = args.single_quoted::<String>()?.parse().unwrap();
    if number_of_players <= 1 {
        return send_embed_message(ctx, msg, WRONG_PLAYERS_NUMBER_MSG).await;
    }

    /* Group can't start a game if there are solo games. */
    if !wordle_map.lock().await.games.is_empty() {
        return send_embed_message(ctx, msg, SOLO_PLAYING_MSG).await;
    }

    /* If there is a start for a group play, games map will contain
    UserId of a person who initiated a game. */
    add_new_wordle(msg, wordle_map).await;
    wordle_map.lock().await.max_people_playing = number_of_players;
    wordle_map.lock().await.joined_people.push(msg.author.id);
    send_embed_message(ctx, msg, WAIT_FOR_PLAYERS_MSG).await
}

fn check_channel(wordle_map: &MutexGuard<'_, ServerMap>, msg: &Message) -> bool {
    for &key in wordle_map.games.keys() {
        if key.0 == msg.channel_id {
            return true;
        }
    }
    false
}

/* Changes time for now in a games map. */
fn change_time(wordle_map: &mut MutexGuard<'_, ServerMap>) {
    for (_, (_, time)) in wordle_map.games.iter_mut() {
        *time = SystemTime::now();
    }
}

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let mut wordle_data = ctx.data.write().await;
    let mut wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordle map!")
        .lock()
        .await;

    check_ended_games(&mut wordle_map);

    /* No one can join if no one initiated a group game. */
    if wordle_map.max_people_playing == 1 {
        return send_embed_message(ctx, msg, START_GROUP_MSG).await;
    }

    if wordle_map.joined_people.len() == wordle_map.max_people_playing {
        return send_embed_message(ctx, msg, GROUP_PLAYING_MSG).await;
    }

    if !check_channel(&wordle_map, msg) {
        return send_embed_message(ctx, msg, WRONG_CHANNEL_MSG).await;
    }

    if wordle_map.joined_people.contains(&msg.author.id) {
        return send_embed_message(ctx, msg, ALREADY_JOINED_MSG).await;
    }

    wordle_map.joined_people.push(msg.author.id);
    if wordle_map.joined_people.len() != wordle_map.max_people_playing {
        return send_embed_message(
            ctx,
            msg,
            &format!(
                "You successfully joined the group! To start the game wait for {} other people",
                wordle_map.max_people_playing - wordle_map.joined_people.len()
            ),
        )
        .await;
    }

    /* If there are enough people in a group, reset the timer and start the game. */
    change_time(&mut wordle_map);
    send_embed_message(ctx, msg, GAME_STARTED_MSG).await
}

fn clean_game(wordle_map: &mut MutexGuard<'_, ServerMap>, msg: &Message, author: UserId) {
    wordle_map.games.remove(&(msg.channel_id, author));
    wordle_map.joined_people.clear();
    wordle_map.max_people_playing = 1;
}

#[command]
async fn guess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut wordle_data = ctx.data.write().await;
    let mut wordle_map = wordle_data
        .get_mut::<ServerKey>()
        .expect("Failed to retrieve wordle map!")
        .lock()
        .await;

    check_ended_games(&mut wordle_map);

    let mut author = msg.author.id;
    let mut is_group = false;

    /* Check if a person can guess if there is a group game. */
    if wordle_map.max_people_playing > 1 {
        is_group = true;
        if wordle_map.joined_people.len() != wordle_map.max_people_playing {
            return send_embed_message(
                ctx,
                msg,
                &format!(
                    "To start the game wait for {} other people",
                    wordle_map.max_people_playing - wordle_map.joined_people.len()
                ),
            )
            .await;
        } else if !check_channel(&wordle_map, msg) {
            return send_embed_message(ctx, msg, GUESS_WRONG_CHANNEL_MSG).await;
        } else if !wordle_map.joined_people.contains(&msg.author.id) {
            return send_embed_message(ctx, msg, NOT_IN_GROUP_MSG).await;
        }
        author = wordle_map.joined_people[0];
    }

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
        return send_embed_message(ctx, msg, INCORRECT_GUESS_MSG).await;
    }
    if !words_vector.contains(&guess) {
        return send_embed_message(ctx, msg, NOT_IN_LIST_MSG).await;
    }

    let wordle = wordle_map.games.get_mut(&(msg.channel_id, author));
    if wordle.is_none() {
        return send_embed_message(ctx, msg, START_PLAYING_MSG).await;
    }

    let mut wordle = &mut wordle.unwrap().0;
    wordle.guesses += 1;

    /* Check if the guess was correct or if the person ran out of guesses.
    If not, add guess to the list and display all guesses. */
    if guess.eq(&wordle.word) {
        string_response.append(WON_MSG);
        clean_game(&mut wordle_map, msg, author);
    } else if wordle.guesses == GUESSES {
        string_response.append(TOO_MANY_GUESSES_MSG);
        string_response.append(wordle.word.as_str());
        clean_game(&mut wordle_map, msg, author);
    } else {
        wordle.add_fields(guess);
        if !is_group {
            string_response.append(msg.author.name.to_string());
            string_response.append(YOUR_GUESSES_MSG);
        }
        wordle.display_game(&mut string_response);
        string_response.append(GUESS_AGAIN);
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
#[commands(start, guess, help, join)]
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
