use std::fmt::Display;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod list;
mod prepare;
mod solutions;
mod solve;

use list::CmdList;
use prepare::CmdPrepare;
use solutions::CmdSolutions;
use solve::CmdSolve;

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
    /// Prepare word list
    Prepare(CmdPrepare),
    /// Solve word puzzle
    Solve(CmdSolve),
    /// List available word lists
    List(CmdList),
    /// Report multiple solutions for the puzzle
    Solutions(CmdSolutions),
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Prepare(_) => write!(f, "prepare"),
            Commands::Solutions(_) => write!(f, "solutions"),
            Commands::Solve(_) => write!(f, "solve"),
            Commands::List(_) => write!(f, "list"),
        }
    }
}
