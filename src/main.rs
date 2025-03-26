use std::process::exit;

use clap::Parser as _;
use slb::{Cli, LettersBoxed, PrepareWords};
use tracing::info;
use tracing_subscriber::EnvFilter;

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "mit_words.txt";
const DEFAULT_MINIMUM_WORD_LENGTH: usize = 3;

fn main() {
    let args = Cli::parse();
    get_logging(args.logging.log_level_filter());

    tracing::debug!("Args: {args:#?}");

    let mut letters = Vec::new();

    if !args.letters.len() == 12 {
        tracing::error!(
            "Must supply exactly 12 letters, {} letters provided.",
            args.letters.len()
        );
        exit(1);
    }

    if !args.letters.is_empty() {
        letters = args.letters;
    }

    // Setup settings
    let src_directory = args.dir.unwrap_or(DEFAULT_SOURCE_DIR.to_string());
    let src_file = args.file.unwrap_or(DEFAULT_SOURCE_FILE.to_string());
    let minimum_word_length = args.minimum.unwrap_or(DEFAULT_MINIMUM_WORD_LENGTH);

    let src = format!("{src_directory}/{src_file}");

    let mut words = Vec::new();

    for line in std::fs::read_to_string(src)
        .expect("Failed to read words file")
        .lines()
    {
        if !line.is_empty() {
            let ws = line.split_whitespace();
            for w in ws {
                words.push(w.to_string());
            }
        }
    }

    tracing::info!("Loaded {} words", words.len());
    let words = words
        .filter_to_minimum_length(minimum_word_length)
        .filter_no_repeated_letters();

    tracing::info!("Filtered words includes {} words", words.len());

    let mut puzzle = LettersBoxed::new(letters, words);
    match puzzle
        .filter_words_with_letters_only()
        .filter_words_with_invalid_pairs()
        .build_word_chain()
    {
        Ok(_) => {
            tracing::info!("Word chain built successfully");
        }
        Err(e) => {
            tracing::error!("Failed to build word chain: {}", e);
        }
    };

    println!("Word chain: {}", puzzle.solution_string());
}

pub fn get_logging(verbosity: log::LevelFilter) {
    let filter = EnvFilter::from(format!(
        "slb={}",
        if verbosity == log::LevelFilter::Trace {
            log::LevelFilter::Debug
        } else {
            verbosity
        }
    ));

    let log_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .pretty()
        .with_env_filter(filter)
        .finish();

    let _ = tracing::subscriber::set_global_default(log_subscriber)
        .map_err(|_| eprintln!("Unable to set global default subscriber!"));

    info!("Initialised logging to console at {verbosity}");
}
