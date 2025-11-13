pub mod util;
pub mod generate;
pub mod solve;

use std::{io::Write as _, path::PathBuf, time::Instant};
use clap::{Parser, Subcommand};
use log::Level;
use ocrs::{DecodeMethod, ImageSource, OcrEngine, OcrEngineParams};
use anyhow::Context;
use rten::Model;

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

        width: u8,
        height: u8
    },
    Recognize {
        file: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
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

    let now = Instant::now();

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
        },
        Commands::Recognize { file } => {
            let detection_model_path = file_path("text-detection.rten");
            let rec_model_path = file_path("text-recognition.rten");

            let detection_model = Model::load_file(detection_model_path)?;
            let recognition_model = Model::load_file(rec_model_path)?;

            let engine = OcrEngine::new(OcrEngineParams {
                detection_model: Some(detection_model),
                recognition_model: Some(recognition_model),
                ..Default::default()
            })?;

            let img = image::open(file)?;
            let ocr_img = img.as_rgb8().unwrap_or_else(|| { log::error!("File not found"); std::process::exit(-1); });
            let src = ImageSource::from_bytes(ocr_img.as_raw(), ocr_img.dimensions())?;
            let inp = engine.prepare_input(src)?;
            let txt = engine.get_text(&inp)?.replace(" ", "");
            let result = txt.lines().enumerate().map(|(i, l)| if i == txt.lines().count()-1 { l.to_string() } else { format!("{};", l) }).collect::<Vec<_>>().join("\n");

            log::info!("Text found in image: {result}");
        }
    }

    log::info!("elapsed: {:#?}", now.elapsed());

    Ok(())
}

fn file_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    abs_path
}

