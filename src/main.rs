pub mod util;

use std::io::Write as _;
use clap::{Parser, Subcommand};
use log::Level;

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

            let mut has_all_words = true;

            for word in words {
                if !find_word(&grid, &word) {
                    log::warn!("Word not found: {word}");
                    has_all_words = false;
                }
            }

            if !has_all_words {
                log::warn!("Didn't find every specified word! Terminating...");
                std::process::exit(-1);
            }
        }
    }
}

fn find_word(grid: &Vec<Vec<char>>, word: &str) -> bool {
    if word.is_empty() {
        return true;
    }

    let first_char = word.chars().next().unwrap();
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
            if search_from(&grid, word, r, c, dr, dc) {
                log::info!("Found word: '{word}' starting at ({c}, {r}) going {place_type:?}");
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

