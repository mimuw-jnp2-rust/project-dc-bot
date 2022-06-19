use crate::wordle::{DEFAULT_SIZE, GUESSES};
use crate::Wordle;
use const_format::formatcp;
use serde_json::{json, Value};
use serenity::framework::standard::CommandResult;
use serenity::http::Http;
use serenity::model::prelude::{ChannelId, Message, ReactionType};
use serenity::prelude::{Context, SerenityError};

use string_builder::Builder;

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
const DICTIONARY_REQUEST: &str = "https://api.dictionaryapi.dev/api/v2/entries/en/";

/* Sends the contents of message_builder to a channel. */
pub async fn send_message(
    http: &Http,
    channel: &ChannelId,
    message_builder: Builder,
) -> Result<Message, SerenityError> {
    channel.say(http, message_builder.string().unwrap()).await
}

/* Adds a white flag reaction under a message.
 * The message is supposed to display the current state of the game. */
pub async fn react_to_message(http: &Http, message: &Message, wordle: &mut Wordle) {
    wordle.last_message_id = Some(message.id);
    if let Err(why) = message
        .react(http, ReactionType::Unicode(String::from("🏳")))
        .await
    {
        println!("Could not react to the message; {}", why);
    }
}

/* Fetches a definition for a given word from a dictionary API.
 * Returns the first definition found.
 * If there has been an error, returns an empty String. */
async fn get_definition(word: &str) -> String {
    let mut definition = String::from("");
    let default = json!("");
    let mut url = String::from(DICTIONARY_REQUEST);
    url.push_str(word);
    let request = reqwest::get(url).await;
    match request {
        Err(why) => {
            println!("Error fetching the definition: {}", why)
        }
        Ok(response) => {
            if let Err(why) = response
                .json::<Value>()
                .await
                .map(|json_value| {
                    json_value
                        .pointer("/0/meanings/0/definitions/0/definition")
                        .unwrap_or(&default)
                        .clone()
                })
                .map(|value| {
                    definition = value.to_string();
                })
            {
                println!("Error reading the definition: {}", why);
            }
        }
    }
    definition
}

/* Sends the solution to given Wordle to the given channel.
 * The channel is supposed to be the same one in which the game is happening. */
pub async fn send_wordle_solution(wordle: &Wordle, channel: &ChannelId, http: &Http) {
    let definition = get_definition(wordle.word.as_str()).await;

    if let Err(why) = channel
        .send_message(http, |m| {
            m.content("Your word was:")
                .embed(|e| e.title(wordle.word.as_str()).description(definition))
        })
        .await
    {
        println!("Error sending the message: {}", why);
    }
}

pub async fn send_embed_message(ctx: &Context, msg: &Message, message: &str) -> CommandResult {
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
