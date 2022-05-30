use std::process::Output;
use serde::Deserialize;
use bracket_random::prelude::RandomNumberGenerator;
use serenity::futures::future::MaybeDone::Future;
use serenity::futures::{StreamExt, TryStreamExt};
use tokio::task::spawn_blocking;

#[derive(Deserialize)]
pub struct Word {
    word: String,
}

pub struct Words {
    words: Vec<Word>,
}

impl Word {
    pub fn word(&self) -> String {
        self.word.clone()
    }
}

impl Words {
    pub async fn new() -> Words {
        let result = reqwest::get("https://raw.githubusercontent.com/mongodb-developer/bash-wordle/main/words.json")
            .await;

        match result {
            Err(why) => {
                println!("Error fetching data: {}", why);
                Words { words: vec![Word { word: String::from("empty") }] }
            }
            Ok(response) => {
                Words {
                    words: response.json()
                        .await
                        .or_else(|err| {
                            println!("Parsing error: {}", err);
                            Result::Ok(vec![])
                        })
                        .unwrap()
                }
            }
        }
    }

    pub fn generate_word(&self) -> &Word {
        let mut rng = RandomNumberGenerator::new();
        rng.random_slice_entry(&self.words).unwrap().clone()
    }
}
