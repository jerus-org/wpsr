use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
pub struct Cli {
    pub letters: Vec<char>,
    /// logging level
    #[clap(flatten)]
    pub logging: Verbosity,
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
    /// word list source file
    #[arg(short, long)]
    pub file: Option<String>,
    /// minimum word length
    #[arg(short, long)]
    pub minimum: Option<String>,
}
