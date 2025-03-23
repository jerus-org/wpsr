use slb::{LettersBoxed, PrepareWords};
use tracing::info;
use tracing_subscriber::EnvFilter;

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "mit_words.txt";
const DEFAULT_MINIMUM_WORD_LENGTH: usize = 3;

fn main() {
    println!("Hello, world!");
    // Setup logging/tracing

    get_logging(log::LevelFilter::Debug);

    // Setup settings
    let src_directory = DEFAULT_SOURCE_DIR;
    let src_file = DEFAULT_SOURCE_FILE;
    let minimum_word_length = DEFAULT_MINIMUM_WORD_LENGTH;

    let src = format!("{src_directory}/{src_file}");

    let words: Vec<String> = std::fs::read_to_string(src)
        .expect("Failed to read words file")
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    tracing::info!("Loaded {} words", words.len());
    let words = words
        .filter_to_minimum_length(minimum_word_length)
        .filter_no_repeated_letters();

    tracing::info!("Filtered words includes {} words", words.len());

    let letters = vec![];
    let mut puzzle = LettersBoxed::new(letters, words);
    puzzle.filter_words_with_letters_only().filter_words_with_invalid_pairs();
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
