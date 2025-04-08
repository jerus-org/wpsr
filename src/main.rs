use std::{collections::HashMap, path::PathBuf, str::FromStr};

use clap::Parser;
use config::{Config, File};
use lib_wpsr::{
    Cli, Commands, DEFAULT_BOXED_SOURCE_FILE, DEFAULT_CONFIG_FILE_BASENAME, DEFAULT_LINE_LENGTH,
    DEFAULT_MINIMUM_WORD_LENGTH, DEFAULT_SOURCE_DIR, DEFAULT_WORDS_SOURCE_FILE,
};
use tracing_subscriber::EnvFilter;

// const DEFAULT_SOURCE_DIR: &str = "/usr/lib/wpsr/words";
// const DEFAULT_SOURCE_FILE: &str = "default.slb";
// const DEFAULT_MINIMUM_WORD_LENGTH: usize = 3;
// const DEFAULT_LINE_LENGTH: usize = 3010;
// const DEFAULT_CONFIG_FILE_BASENAME: &str = "wpsr";

fn main() {
    let args = Cli::parse();
    get_logging(args.logging.log_level_filter());

    let base_name = DEFAULT_CONFIG_FILE_BASENAME;

    match get_settings(base_name) {
        Ok(settings) => {
            let settings = settings
                .try_deserialize::<HashMap<String, String>>()
                .unwrap();
            tracing::debug!("Loaded settings: {:?}", settings);
            tracing::debug!("Args: {args:#?}");
            let res = match args.cmd {
                Commands::Boxed(boxed) => boxed.run(settings),
                Commands::List(list) => list.run(settings),
                Commands::Words(words) => words.run(settings),
                Commands::Alpha(alpha) => alpha.run(settings),
            };
            match res {
                Ok(_) => {
                    tracing::info!("Done");
                }
                Err(e) => {
                    tracing::error!("Failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to load settings: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn get_logging(verbosity: log::LevelFilter) {
    let filter = EnvFilter::from(format!(
        "wpsr={},lib_wpsr={}",
        if verbosity == log::LevelFilter::Trace {
            log::LevelFilter::Debug
        } else {
            verbosity
        },
        if verbosity == log::LevelFilter::Trace {
            log::LevelFilter::Debug
        } else {
            verbosity
        }
    ));

    let log_subscriber = tracing_subscriber::FmtSubscriber::builder()
        // .pretty()
        .compact()
        .with_env_filter(filter)
        .finish();

    let _ = tracing::subscriber::set_global_default(log_subscriber)
        .map_err(|_| eprintln!("Unable to set global default subscriber!"));

    tracing::info!("Initialised logging to console at {verbosity}");
}

#[tracing::instrument]
pub fn get_settings(base_name: &str) -> Result<Config, config::ConfigError> {
    let path = PathBuf::from_str(&format!("{}.toml", base_name)).unwrap();
    tracing::debug!("Loading settings from {}", path.display());
    if path.exists() {
        tracing::debug!("Settings file exists");
        Config::builder()
            .set_default("source_dir", DEFAULT_SOURCE_DIR)?
            .set_default("source_words_file", DEFAULT_WORDS_SOURCE_FILE)?
            .set_default("source_boxed_file", DEFAULT_BOXED_SOURCE_FILE)?
            .set_default(
                "minimum_word_length",
                DEFAULT_MINIMUM_WORD_LENGTH.to_string(),
            )?
            .set_default("line_length", DEFAULT_LINE_LENGTH.to_string())?
            .add_source(File::with_name(base_name))
            .build()
    } else {
        tracing::debug!("Settings file does not exist");
        Config::builder()
            .set_default("source_dir", DEFAULT_SOURCE_DIR)?
            .set_default("source_words_file", DEFAULT_WORDS_SOURCE_FILE)?
            .set_default("source_boxed_file", DEFAULT_BOXED_SOURCE_FILE)?
            .set_default(
                "minimum_word_length",
                DEFAULT_MINIMUM_WORD_LENGTH.to_string(),
            )?
            .set_default("line_length", DEFAULT_LINE_LENGTH.to_string())?
            .build()
    }
}
