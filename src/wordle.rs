use std::collections::HashMap;

pub enum Result {
    Green,
    Yellow,
    Red,
}
pub static DEFAULT_SIZE: usize = 5;
pub static GUESSES: u32 = 6;
pub static GREEN_SQUARE: &str = ":green_square: ";
pub static YELLOW_SQUARE: &str = ":yellow_square: ";
pub static RED_SQUARE: &str = ":red_square: ";

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

    pub fn display_game(&self, string_response: &mut String) {
        for round in 1..(GUESSES + 1) {
            if self.guesses >= round {
                let vec_fields = self.fields.get(&round).unwrap();
                /* Displays guessed word. */
                for field in vec_fields {
                    string_response.push_str(&format!("{}     ", field.letter));
                }
                string_response.push('\n');

                /* Displays different colored squares depending on square value in Field. */
                for field in vec_fields {
                    match field.square {
                        Result::Red => {
                            string_response.push_str(RED_SQUARE);
                        }
                        Result::Yellow => {
                            string_response.push_str(YELLOW_SQUARE);
                        }
                        Result::Green => {
                            string_response.push_str(GREEN_SQUARE);
                        }
                    }
                }
                string_response.push('\n');
            }
        }
    }
}
