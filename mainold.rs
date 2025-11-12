pub mod util;

use std::io::Write as _;
use clap::{Parser, Subcommand};
use enum_derived::Rand;
use log::Level;
use rand::Rng;

use crate::util::PlaceType;

#[derive(Parser)]
#[command(author="Illia Zhdanov", version="0.1", about="CrossVault - the *fastest* crossword solver")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        #[arg(short, long, value_delimiter = ';')]
        crossword: Vec<String>,

        #[arg(short, long, value_delimiter = ',')]
        words: Vec<String>
    },
    Generate {
        #[arg(short, long)]
        diagonal: bool,
        #[arg(short, long)]
        reverse: bool,
        #[arg(short, long)]
        grid_width: u8,
        #[arg(short, long)]
        grid_height: u8
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level_color = match record.level() {
                Level::Trace => "\x1b[90m",   // Bright black / gray
                Level::Debug => "\x1b[34m",   // Blue
                Level::Info  => "\x1b[32m",   // Green
                Level::Warn  => "\x1b[33m",   // Yellow
                Level::Error => "\x1b[31m",   // Red
            };
            let reset = "\x1b[0m";

            writeln!(buf, "[{level_color}{}{reset}] ({}) {}", record.level(), record.target(), record.args())
        })
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Solve { crossword, words } => {
            log::debug!("crossword: {:#?}, words: {:#?}", crossword, words);

            let grid: Vec<Vec<char>> = crossword
                .iter()
                .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect())
                .collect();

            let mut not_found_words = vec![];
            for word in &words {
                if !find_word_directionally(&grid, word, false) {
                    not_found_words.push(word.to_string());
                }
            }

            if !not_found_words.is_empty() {
                log::info!("Trying to find unfound words in reverse...");
                let mut still_not_found_words = vec![];
                for word in not_found_words {
                    if !find_word_directionally(&grid, &word, true) {
                        still_not_found_words.push(word);
                    }
                }

                if !still_not_found_words.is_empty() {
                    for word in still_not_found_words {
                        log::warn!("Word not found: {}", word);
                    }
                    log::warn!("Didn't find every specified word! Terminating...");
                    std::process::exit(-1);
                }
            }
        },
        Commands::Generate { diagonal, reverse, grid_width, grid_height } => {
            generate_wordsearch(diagonal, reverse, grid_width, grid_height);
        }
    }
}

fn generate_wordsearch(allow_diagonal: bool, allow_reverse: bool, grid_width: u8, grid_height: u8) {
    let mut rng = rand::rng();
    let place_type: PlaceType = PlaceType::rand();

    let word_amount: u8 = rng.random_range((grid_width as f32 * grid_height as f32 / 30.0).ceil() as u8..=((grid_width * grid_height) / 10) as u8);

    log::info!("word_amount: {word_amount}, place_Type: {place_type:#?}");
}

fn find_word_directionally(grid: &Vec<Vec<char>>, word: &str, reversed: bool) -> bool {
    let search_word = if reversed {
        word.chars().rev().collect::<String>()
    } else {
        word.to_string()
    };

    let first_char = search_word.chars().next().unwrap();
    let indices = calculate_indices(grid, first_char);

    for (r, c) in indices {
        let directions = vec![
            (-1, -1, PlaceType::UpperLeftDiagonal),
            (-1, 0, PlaceType::UpStraight),
            (-1, 1, PlaceType::UpperRightDiagonal),
            (0, -1, PlaceType::LeftStraight),
            (0, 1, PlaceType::RightStraight),
            (1, -1, PlaceType::LowerLeftDiagonal),
            (1, 0, PlaceType::DownStraight),
            (1, 1, PlaceType::LowerRightDiagonal)
        ];

        for (dr, dc, place_type) in directions {
            if search_from(&grid, &search_word, r, c, dr, dc) {
                log::info!("Found word: '{word}' starting at ({c}, {r}) going {place_type:?}{}", if reversed { " (reversed)" } else { "" });
                return true;
            }
        }
    }
    false
}

fn search_from(grid: &Vec<Vec<char>>, word: &str, r: usize, c: usize, dr: i32, dc: i32) -> bool {
    let mut chars = word.chars();
    let mut r = r as isize;
    let mut c = c as isize;

    while let Some(char_to_find) = chars.next() {
        if r < 0 || c < 0 || r as usize >= grid.len() || c as usize >= grid[0].len() {
            return false;
        }

        let grid_char = grid[r as usize][c as usize];
        if grid_char.to_ascii_lowercase() != char_to_find.to_ascii_lowercase() {
            return false;
        }

        r += dr as isize;
        c += dc as isize;
    }

    true
}


pub fn calculate_indices(grid: &Vec<Vec<char>>, target: char) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &c)| {
                    if c.to_ascii_lowercase() == target.to_ascii_lowercase() {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
    .collect()
}

