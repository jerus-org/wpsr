use std::fmt::Display;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod boxed;
mod list;

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
    /// Boxed word puzzle tools
    Boxed(boxed::Cmd),
    /// List available word lists
    List(list::Cmd),
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Boxed(_) => write!(f, "boxed"),
            Commands::List(_) => write!(f, "list"),
        }
    }
}
