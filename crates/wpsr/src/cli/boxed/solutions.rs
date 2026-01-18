use std::collections::HashMap;

use crate::{Error, Shuffle, Solution};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    pub letters: String,
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
    /// word list source file
    #[arg(short, long)]
    pub file: Option<String>,
    /// number of random solutions to generate
    #[arg(short, long, default_value_t = 100)]
    pub random_solutions: usize,
    /// maximum length of the word chain
    #[arg(short, long, default_value_t = 10)]
    pub max_chain: usize,
    /// Shuffle depth
    #[arg(short, long, default_value_t = 3)]
    pub shuffle_depth: i8,
}

impl Cmd {
    #[tracing::instrument(skip(self, settings))]
    pub fn run(&self, settings: HashMap<String, String>) -> Result<(), Error> {
        tracing::debug!("Args: {self:#?}");

        let mut solution = Solution::new(&self.letters, settings)?;
        solution
            .set_word_source(self.dir.clone(), self.file.clone())
            .load_words()
            .set_max_chain(self.max_chain)
            .set_shuffle_depth(self.shuffle_depth)
            .find_best_solution()?;

        let mut max_clashes = 10;
        let mut max_solutions = self.random_solutions;

        let bar = indicatif::ProgressBar::new(max_solutions as u64);
        while max_solutions > 0 && max_clashes > 0 {
            tracing::info!(
                "Generating random solutions max_solutions={max_solutions} and max_clashes={max_clashes}"
            );
            match solution.find_random_solution(Shuffle::Once) {
                Ok(_) => {
                    max_solutions -= 1;
                    bar.inc(1);
                }
                Err(Error::SolutionAlreadyFound) => {
                    max_clashes -= 1;
                }
                Err(e) => {
                    tracing::error!("Failed to build word chain: {}", e);
                }
            };
        }
        bar.finish();

        println!("{}", solution.solutions_title());
        println!("{}\n", solution.word_source_string());
        println!("{}", solution.distribution_string());
        println!("{}", solution.solutions_string());

        Ok(())
    }
}
