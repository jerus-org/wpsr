mod anagram;
mod boxed;
mod cli;
mod error;
mod word_filters;
mod words;

pub use anagram::Anagram;
pub use boxed::{LettersBoxed, Shape, Shuffle, Solution};
pub use cli::{Cli, Commands};
pub use error::Error;
pub use word_filters::WordFilters;
pub use words::Words;

pub const DEFAULT_SOURCE_DIR: &str = "/usr/lib/wpsr/words";
pub const DEFAULT_WORDS_SOURCE_FILE: &str = "default.txt";
pub const DEFAULT_BOXED_SOURCE_FILE: &str = "default.slb";
pub const DEFAULT_MINIMUM_WORD_LENGTH: usize = 3;
pub const DEFAULT_LINE_LENGTH: usize = 3010;
pub const DEFAULT_CONFIG_FILE_BASENAME: &str = "wpsr";
