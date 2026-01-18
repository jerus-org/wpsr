use std::collections::HashMap;

use crate::{Anagram, Error};
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
}

impl Cmd {
    #[tracing::instrument(skip(self, settings))]
    pub fn run(&self, settings: HashMap<String, String>) -> Result<(), Error> {
        tracing::debug!("Args: {self:#?}");

        let mut solution = Anagram::new(&self.letters, settings)?;
        solution
            .set_word_source(self.dir.clone(), self.file.clone())
            .load_words()
            .find_solutions()?;

        println!("{}", solution.solutions_title());
        println!("{}\n", solution.word_source_string());
        println!("{}", solution.solutions_string());
        println!("{}", solution.distribution_string());

        Ok(())
    }
}
