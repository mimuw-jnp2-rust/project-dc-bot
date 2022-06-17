use std::collections::HashMap;
use string_builder::Builder;

pub enum Result {
    Green,
    Yellow,
    Red,
}

pub const DEFAULT_SIZE: usize = 5;
pub const GUESSES: u32 = 6;
pub const GREEN_SQUARE: &str = ":green_square: ";
pub const YELLOW_SQUARE: &str = ":yellow_square: ";
pub const RED_SQUARE: &str = ":red_square: ";

/* Struct representing a single char in guess word. */
pub struct Field {
    pub letter: char,
    pub square: Result,
}

impl Field {
    pub fn new(letter: char, square: Result) -> Field {
        Field { letter, square }
    }
}

/* Struct representing a single instance of the game. */
pub struct Wordle {
    pub word: String,
    pub guesses: u32,
    pub fields: HashMap<u32, Vec<Field>>,
}

impl Wordle {
    pub fn new(word: String) -> Wordle {
        Wordle {
            word,
            guesses: 0,
            fields: HashMap::new(),
        }
    }

    /* Saves guess word as Fields with corresponding color describing if char
    matches the chars in a word to guess. */
    pub fn add_fields(&mut self, guess: String) {
        let mut field_vec = Vec::new();
        for (pos, c) in guess.chars().enumerate() {
            let field: Field;
            if c == self.word.chars().nth(pos).unwrap() {
                field = Field::new(c, Result::Green);
            } else if self.word.chars().any(|word_c| word_c == c) {
                field = Field::new(c, Result::Yellow);
            } else {
                field = Field::new(c, Result::Red);
            }
            field_vec.push(field);
        }
        self.fields.insert(self.guesses, field_vec);
    }

    pub fn display_game(&self, string_response: &mut Builder) {
        for round in 1..(GUESSES + 1) {
            if self.guesses >= round {
                let vec_fields = self.fields.get(&round).unwrap();
                /* Displays guessed word. */
                for field in vec_fields {
                    string_response.append(format!("{}     ", field.letter));
                }
                string_response.append('\n');

                /* Displays different colored squares depending on square value in Field. */
                for field in vec_fields {
                    match field.square {
                        Result::Red => {
                            string_response.append(RED_SQUARE);
                        }
                        Result::Yellow => {
                            string_response.append(YELLOW_SQUARE);
                        }
                        Result::Green => {
                            string_response.append(GREEN_SQUARE);
                        }
                    }
                }
                string_response.append('\n');
            }
        }
    }
}
