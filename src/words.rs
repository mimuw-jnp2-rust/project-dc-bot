use bracket_random::prelude::RandomNumberGenerator;
use serde::Deserialize;

/* Struct representing a single available word. */
#[derive(Deserialize)]
pub struct Word {
    pub word: String,
}

/* Struct representing all available words to guess. */
pub struct Words {
    pub words: Vec<Word>,
}

impl Words {
    pub async fn new() -> Words {
        let result = reqwest::get(
            "https://raw.githubusercontent.com/mongodb-developer/bash-wordle/main/words.json",
        )
        .await;

        match result {
            Err(why) => {
                println!("Error fetching data: {}", why);
                Words {
                    words: vec![Word {
                        word: String::from("empty"),
                    }],
                }
            }
            Ok(response) => Words {
                words: response.json().await.unwrap(),
            },
        }
    }

    pub fn generate_word(&self) -> &Word {
        let mut rng = RandomNumberGenerator::new();
        rng.random_slice_entry(&self.words).unwrap()
    }
}
