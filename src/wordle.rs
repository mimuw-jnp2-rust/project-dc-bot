use std::collections::HashMap;

pub enum Result {Green, Yellow, Black}
static DEFAULT_SIZE: u32 = 5;
static GUESSES: u32 = 6;

pub struct Field {
    pub letter: char,
    pub square: Result
}

/* Struct representing a single instance of the game. */
pub struct Wordle {
    pub word: String,
    pub guesses: u32,
    pub fields: HashMap<u32, Vec<Field>>
}

impl Wordle {
    pub fn new() -> Wordle {
        Wordle {
            word: String::from("ABCDE"),
            guesses: 0,
            fields: HashMap::new(),
        }
    }
}
