use std::fmt::Display;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod alpha;
mod anagram;
mod boxed;
mod list;
mod words;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// logging level
    #[clap(flatten)]
    pub logging: Verbosity,
    /// Commands to run
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Parse list of words to exclude duplicates and non-alphabetic characters
    Alpha(alpha::Cmd),
    /// List available word lists
    List(list::Cmd),
    /// Find words that are anagrams of a given letter string
    Anagram(anagram::Cmd),
    /// Boxed word puzzle tools
    Boxed(boxed::Cmd),
    /// Generate words from a string of letters
    Words(words::Cmd),
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Alpha(_) => write!(f, "alpha"),
            Commands::List(_) => write!(f, "list"),
            Commands::Anagram(_) => write!(f, "anagram"),
            Commands::Boxed(_) => write!(f, "boxed"),
            Commands::Words(_) => write!(f, "words"),
        }
    }
}
