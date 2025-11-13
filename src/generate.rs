use rand::Rng;
use crate::util::{PlaceType, Word};
use std::{cmp::Ordering, vec::Vec};
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

pub fn random_coords(place_type: PlaceType, word: &str, used_words: &[Word], width: u8, height: u8) -> Option<(u8, u8)> {
    let mut rng = rand::rng();
    let word_len = word.len() as u8;
    
    // Prevent infinite loops - try max 1000 times
    const MAX_ATTEMPTS: u32 = 1000;
    let mut attempts = 0;

    loop {
        attempts += 1;
        if attempts > MAX_ATTEMPTS {
            log::warn!("Could not place word '{}' after {} attempts", word, MAX_ATTEMPTS);
            return None; // Give up
        }

        let x = match place_type {
            PlaceType::RightStraight | PlaceType::LowerRightDiagonal | PlaceType::UpperRightDiagonal => {
                if width < word_len { return None; }
                rng.random_range(0..=width - word_len)
            },
            PlaceType::LeftStraight | PlaceType::LowerLeftDiagonal | PlaceType::UpperLeftDiagonal => {
                if word_len == 0 { return None; }
                rng.random_range(word_len - 1..width)
            },
            _ => rng.random_range(0..width),
        };

        let y = match place_type {
            PlaceType::DownStraight | PlaceType::LowerRightDiagonal | PlaceType::LowerLeftDiagonal => {
                if height < word_len { return None; }
                rng.random_range(0..=height - word_len)
            },
            PlaceType::UpStraight | PlaceType::UpperRightDiagonal | PlaceType::UpperLeftDiagonal => {
                if word_len == 0 { return None; }
                rng.random_range(word_len - 1..height)
            },
            _ => rng.random_range(0..height),
        };

        // Check collision for all cells the word would occupy
        let mut collision = false;
        for i in 0..word_len {
            let (cx, cy) = match place_type {
                PlaceType::RightStraight => (x + i, y),
                PlaceType::LeftStraight => (x - i, y),
                PlaceType::DownStraight => (x, y + i),
                PlaceType::UpStraight => (x, y - i),
                PlaceType::LowerRightDiagonal => (x + i, y + i),
                PlaceType::UpperRightDiagonal => (x + i, y - i),
                PlaceType::LowerLeftDiagonal => (x - i, y + i),
                PlaceType::UpperLeftDiagonal => (x - i, y - i),
            };

            // Check if this position conflicts with any existing word
            if used_words.iter().any(|w| {
                w.positions_and_chars().iter().any(|((wx, wy), _)| *wx == cx && *wy == cy)
            }) {
                collision = true;
                log::debug!("Collision at ({}, {}) for word '{}' attempt {}", cx, cy, word, attempts);
                break;
            }
        }

        if !collision {
            log::debug!("Successfully placed '{}' at ({}, {}) after {} attempts", word, x, y, attempts);
            return Some((x, y));
        }
    }
}

// Updated generate function to handle placement failures
pub fn generate(allow_diagonal: bool, allow_reverse: bool, width: u8, height: u8) {
    let mut rng = rand::rng();

    let area = (width as u16 * height as u16) as f32;

    let density = if area < 100.0 {
        0.02
    } else if area < 300.0 {
        0.03
    } else {
        0.04
    };

    let min_words = (area * density / 2.0).ceil() as u8;
    let max_words = (area * density).ceil() as u8;
    let word_amount = rng.random_range(min_words..=max_words);

    log::debug!("Attempting to place {} words", word_amount);

    let mut used_words: Vec<Word> = vec![];
    let mut failed_placements = 0;

    for attempt in 0..word_amount {
        let mut word = random_word::get(Lang::En);

        // Find unique word
        while used_words.iter().any(|i| i.word == word) {
            word = random_word::get(Lang::En);
        }

        let place_type = random_place_type(allow_diagonal, allow_reverse);

        // Try to place the word
        if let Some((x, y)) = random_coords(place_type.clone(), word, &used_words, width, height) {
            used_words.push(Word { 
                place_type, 
                word: word.to_owned(), 
                x, 
                y 
            });
            log::debug!("Placed word {} of {}: '{}'", attempt + 1, word_amount, word);
        } else {
            failed_placements += 1;
            log::warn!("Failed to place word '{}' (failure {} so far)", word, failed_placements);
        }
    }

    log::info!("Successfully placed {} words ({} failures)", used_words.len(), failed_placements);
    log::info!("Words: {:?}", used_words.iter().map(|w| &w.word).collect::<Vec<_>>());

    generate_wordsearch_with_words(used_words, width, height);
}



pub fn generate_wordsearch_with_words(words: Vec<Word>, width: u8, height: u8) {
    let mut ws: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];

    for word in &words {
        word.apply(&mut ws);
    }

    log::debug!("words: {words:?}");

    fill_empty(&mut ws);
}

pub fn fill_empty(ws: &mut Vec<Vec<char>>) {
    let mut rng = rand::rng();

    for row in ws.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == ' ' {
                *cell = (b'a' + rng.random_range(0..26)) as char;
            }
        }
    }

    let pretty = ws.iter()
        .map(|row| {
            let ch: String = row.iter().map(|ch| ch.to_uppercase().to_string() + " ").collect::<String>();
            ch.strip_suffix(" ").unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join(";\n");

    log::info!("Your generated word-search: \n{pretty}");
}
