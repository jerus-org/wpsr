use std::fmt::Display;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod generate;
mod list;
mod prepare;
mod solutions;
mod solve;

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
    /// Generate random letter string for puzzle
    Generate(generate::Cmd),
    /// List available word lists
    List(list::Cmd),
    /// Prepare word list
    Prepare(prepare::Cmd),
    /// Report multiple solutions for the puzzle
    Solutions(solutions::Cmd),
    /// Solve word puzzle
    Solve(solve::Cmd),
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Generate(_) => write!(f, "generate"),
            Commands::List(_) => write!(f, "list"),
            Commands::Prepare(_) => write!(f, "prepare"),
            Commands::Solutions(_) => write!(f, "solutions"),
            Commands::Solve(_) => write!(f, "solve"),
        }
    }
}
