use rand::Rng;
use crate::util::{PlaceType, Word};
use std::vec::Vec;
use enum_derived::Rand;
use random_word::Lang;

pub fn random_place_type(allow_diagonal: bool, allow_reverse: bool) -> PlaceType {
    let mut place_type: PlaceType = PlaceType::rand();

    if !allow_diagonal {
        while place_type.is_diagonal() {
            place_type = PlaceType::rand();
        }
    }

    if !allow_reverse {
        while place_type.is_reversed() {
            place_type = PlaceType::rand();
        }
    }

    return place_type;
}

pub fn random_coords(place_type: PlaceType, word: &str, used_words: Vec<Word>, width: u8, height: u8) -> (u8, u8) {
    let mut rng = rand::rng();
    let mut x = rng.random_range(0..width);
    let mut y = rng.random_range(0..height);

    if place_type.is_horizontal() || place_type.is_diagonal() {
        while x + word.len() as u8 - 1 < width && used_words.iter().any(|i| i.x == x) {
            x = rng.random_range(0..width);
        }
    }

    if place_type.is_vertical() || place_type.is_diagonal() {
        while y + word.len() as u8 - 1 < height && used_words.iter().any(|i| i.y == y) {
            y = rng.random_range(0..height);
        }
    }

    return (x, y);
}

pub fn generate(allow_diagonal: bool, allow_reverse: bool, width: u8, height: u8) {
    let mut rng = rand::rng();

    let min_words = ((width as u16 * height as u16) as f32 / 30.0).ceil() as u8;
    let max_words = ((width as u16 * height as u16) / 10) as u8;

    let word_amount: u8 = rng.random_range(min_words..=max_words);

    let mut used_words: Vec<Word> = vec![];

    for _ in 0..word_amount {
        let mut word = random_word::get(Lang::En);

        while used_words.iter().any(|i| i.word == word.to_owned()) {
            word = random_word::get(Lang::En);
        }

        let place_type: PlaceType = random_place_type(allow_diagonal, allow_reverse);
        let (x, y) = random_coords(place_type.clone(), word, used_words.clone(), width, height);

        used_words.push(Word { place_type, word: word.to_owned(), x, y });
    }

    log::info!("allow_reverse: {allow_reverse}, allow_diagonal: {allow_diagonal}, word_amount: {word_amount}, used_words: {used_words:?}");

    generate_wordsearch_with_words(used_words);
}


pub fn generate_wordsearch_with_words(words: Vec<Word>) {
    let ws: Vec<Vec<char>> = vec![vec![]];
    for word in words {
    }
}
