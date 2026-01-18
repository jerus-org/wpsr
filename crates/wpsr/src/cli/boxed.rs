use std::{collections::HashMap, fmt::Display};

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::Error;

mod generate;
mod prepare;
mod solutions;
mod solve;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cmd {
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
            Commands::Prepare(_) => write!(f, "prepare"),
            Commands::Solutions(_) => write!(f, "solutions"),
            Commands::Solve(_) => write!(f, "solve"),
        }
    }
}

impl Cmd {
    pub fn run(self, settings: HashMap<String, String>) -> Result<(), Error> {
        match self.cmd {
            Commands::Generate(generate) => generate.run(settings),
            Commands::Prepare(prepare) => prepare.run(settings),
            Commands::Solutions(solutions) => solutions.run(settings),
            Commands::Solve(solve) => solve.run(settings),
        }
    }
}
