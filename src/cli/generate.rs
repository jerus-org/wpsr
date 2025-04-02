use std::collections::HashMap;

use crate::{Error, Shape};
// use crate::{LettersBoxed, Shuffle};
use clap::Parser;
use rand::seq::SliceRandom;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    pub shape: Shape,
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
            println!("Letters for edges of the {} are: `{}`", self.shape, letters);
        }

        Ok(())
    }
}
