pub mod util;
pub mod generate;
pub mod solve;

use std::io::Write as _;
use clap::{Parser, Subcommand};
use enum_derived::Rand;
use log::Level;
use rand::Rng;
use random_word::Lang;

use crate::util::{PlaceType, Word};

#[derive(Parser)]
#[command(author="Illia Zhdanov", version="0.1", about="CrossVault - the *fastest* crossword solver", disable_help_flag=true)]
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
        #[arg(short, long, required=true)]
        width: u8,
        #[arg(short, long, required=true)]
        height: u8
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
        Commands::Solve { mut crossword, words } => {
            crossword.retain(|inner| !inner.is_empty());

            if crossword.is_empty() {
                log::warn!("Wordsearch is empty... terminating");
            }

            if let Some(last) = crossword.last_mut() {
                if last.ends_with(';') {
                    last.pop(); // removes the last character
                }
            } else {
                panic!("..?");
            }

            let grid: Vec<Vec<char>> = crossword
                .iter()
                .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect())
                .collect();

            let words = words.iter().map(|i| i.chars().filter(|c| !c.is_whitespace()).collect()).collect::<Vec<String>>();

            log::debug!("crossword: {:#?}, words: {:#?}", grid, words);

            let mut not_found_words = vec![];
            for word in &words {
                if !solve::find_word_directionally(&grid, word, false) {
                    not_found_words.push(word.to_string());
                }
            }

            if !not_found_words.is_empty() {
                for word in not_found_words {
                    log::warn!("Word not found: {}", word);
                }
                log::warn!("Didn't find every specified word! Terminating...");
                std::process::exit(-1);
            }
        },
        Commands::Generate { diagonal, reverse, width, height } => {
            generate::generate(diagonal, reverse, width, height);
        }
    }
}

