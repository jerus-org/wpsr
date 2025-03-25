use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
pub struct Cli {
    pub letters: Vec<char>,
    /// logging level
    #[clap(flatten)]
    pub logging: Verbosity,
}
