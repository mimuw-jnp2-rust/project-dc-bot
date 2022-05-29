use std::collections::HashMap;

enum Result {Green, Yellow, Black}
static DEFAULT_SIZE: u32 = 5;
static GUESSES: u32 = 6;

struct Field {
    letter: char,
    square: Result
}

/* Struct representing a single instance of the game. */
pub struct Wordle {
    word: String,
    size: u32,
    guesses: u32,
    fields: HashMap<u32, Vec<Field>>
}

impl Wordle {

    fn new(size: u32) -> Wordle {
        /* TODO randomly picking a word of given size */
        !unimplemented!()
    }
}