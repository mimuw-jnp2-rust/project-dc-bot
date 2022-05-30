use serde::Deserialize;
use bracket_random::prelude::RandomNumberGenerator;
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
        let response = reqwest::blocking::get("https://raw.githubusercontent.com/mongodb-developer/bash-wordle/main/words.json").unwrap();
        Words {
            words: response.json().unwrap(),
        }
    }

    pub fn generate_word(&self) -> &Word {
        let mut rng = RandomNumberGenerator::new();
        rng.random_slice_entry(&self.words).unwrap().clone()
    }
}
