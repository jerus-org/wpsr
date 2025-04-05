use std::collections::HashMap;

use crate::{Error, Shape, Shuffle, Solution};
// use crate::{LettersBoxed, Shuffle};
use clap::Parser;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    pub shape: Shape,
    // Bare result listing the letters for the puzzle only
    #[arg(short, long)]
    pub bare: bool,
    // Testing
    #[arg(long, hide = true)]
    pub testing: bool,
}

impl Cmd {
    #[tracing::instrument(skip(self))]
    pub fn run(self, settings: HashMap<String, String>) -> Result<(), Error> {
        let mut letter_pool = ALPHABET.chars().collect::<Vec<char>>();
        letter_pool.shuffle(&mut rand::rng());

        let letters = letter_pool
            .iter()
            .take(self.shape.edges() as usize * 3)
            .collect::<String>();

        if !self.testing {
            if self.bare {
                println!("{}", letters);
            } else {
                println!("Letters for edges of the {} are: `{}`", self.shape, letters);
            }
        }

        let mut solution = Solution::new(&letters, settings)?;
        solution
            .set_word_source(None, None)
            .load_words()
            .find_best_solution()?;

        let mut max_clashes = 10;
        let mut max_solutions = 100;

        let bar = ProgressBar::new(max_solutions as u64);
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
                    max_solutions -= 1;
                }
            };
        }

        println!("Solutions for {}:", solution.shape_string());
        println!("{}", solution.distribution_string());
        println!("{}", solution.solutions_string());

        Ok(())
    }
}
