mod cli;
mod error;
mod prepare_words;
mod shape;
mod solution;

pub use cli::{Cli, Commands};
pub use error::Error;
pub use prepare_words::PrepareWords;
pub use shape::Shape;
pub use solution::Solution;

pub const DEFAULT_SOURCE_DIR: &str = "/usr/lib/slb/words";
pub const DEFAULT_WORDS_SOURCE_FILE: &str = "default.txt";
pub const DEFAULT_BOXED_SOURCE_FILE: &str = "default.slb";
pub const DEFAULT_MINIMUM_WORD_LENGTH: usize = 3;
pub const DEFAULT_LINE_LENGTH: usize = 3010;
pub const DEFAULT_CONFIG_FILE_BASENAME: &str = "slb";
