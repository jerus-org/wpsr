use std::collections::HashMap;

use clap::Parser;

use crate::{Error, Solution, solution::Shuffle};

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    pub letters: String,
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
    /// word list source file
    #[arg(short, long)]
    pub file: Option<String>,
    /// maximum length of the word chain
    #[arg(short, long, default_value_t = 10)]
    pub max_chain: usize,
    /// Shuffle strategy
    #[arg(
        short,
        long,
        default_value_t = Shuffle::None,
        long_help = "Shuffle strategy\n\nNone - No shuffling\nOnce - Shuffle the weighted list only\nTwice - Shuffle the whole word list and the weighted list"
    )]
    pub shuffle: Shuffle,
    /// Shuffle depth
    #[arg(short, long, default_value_t = 3)]
    pub layers: i8,
}

impl Cmd {
    pub fn run(self, settings: HashMap<String, String>) -> Result<(), Error> {
        tracing::debug!("Args: {self:#?}");

        let mut solution = Solution::new(&self.letters, settings)?;
        solution
            .set_word_source(self.dir.clone(), self.file.clone())
            .load_words()
            .set_max_chain(self.max_chain)
            .set_shuffle_depth(self.layers)
            .find_random_solution(self.shuffle)?;

        println!("{}", solution.solve_title());
        println!("{}\n", solution.word_source_string());
        println!("{}", solution.solutions_string());
        Ok(())
    }
}
