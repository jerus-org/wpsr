use std::{collections::HashMap, path::PathBuf, str::FromStr};

use clap::Parser;
use config::{Config, File};
use slb::{Cli, Commands};
use tracing::info;
use tracing_subscriber::EnvFilter;

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "mit_words.slb";
const DEFAULT_MINIMUM_WORD_LENGTH: usize = 3;
const DEFAULT_LINE_LENGTH: usize = 3010;
const DEFAULT_CONFIG_FILE_BASENAME: &str = "slb";

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
            match args.cmd {
                Commands::Prepare(prepare) => prepare.run(settings),
                Commands::Solve(solve) => solve.run(settings),
                Commands::List(list) => list.run(settings),
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
        "slb={}",
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

    info!("Initialised logging to console at {verbosity}");
}

pub fn get_settings(base_name: &str) -> Result<Config, config::ConfigError> {
    let path = PathBuf::from_str(&format!("{}.toml", base_name)).unwrap();
    if path.exists() {
        Config::builder()
            .set_default("source_dir", DEFAULT_SOURCE_DIR)?
            .set_default("source_file", DEFAULT_SOURCE_FILE)?
            .set_default(
                "minimum_word_length",
                DEFAULT_MINIMUM_WORD_LENGTH.to_string(),
            )?
            .set_default("line_length", DEFAULT_LINE_LENGTH.to_string())?
            .add_source(File::with_name(base_name))
            .build()
    } else {
        Config::builder()
            .set_default("source_dir", DEFAULT_SOURCE_DIR)?
            .set_default("source_file", DEFAULT_SOURCE_FILE)?
            .set_default(
                "minimum_word_length",
                DEFAULT_MINIMUM_WORD_LENGTH.to_string(),
            )?
            .set_default("line_length", DEFAULT_LINE_LENGTH.to_string())?
            .build()
    }
}
