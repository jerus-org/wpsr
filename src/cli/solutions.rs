use std::{collections::HashMap, process::exit};

use crate::{LettersBoxed, Shuffle};
use clap::Parser;

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "mit_words.slb";

#[derive(Parser, Debug, Clone)]
pub struct CmdSolutions {
    pub letters: String,
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
    /// word list source file
    #[arg(short, long)]
    pub file: Option<String>,
    /// number of random solutions to generate
    #[arg(short, long, default_value_t = 50)]
    pub random_solutions: usize,
    /// maximum length of the word chain
    #[arg(short, long, default_value_t = 6)]
    pub max_chain: usize,
    // /// do not shuffle the words
    // #[arg(short, long)]
    // pub no_shuffle: bool,
    // /// number of iterations to shuffle
    // #[arg(short, long)]
    // pub shuffles: Option<usize>,
    // /// shuffle the whole word list and weighted list
    // #[arg(
    //     short,
    //     long,
    //     long_help = "Shuffle the whole word list before calculating weightings\nthen shuffle the top half of the weighted word list."
    // )]
    // pub twice: bool,
}

impl CmdSolutions {
    #[tracing::instrument(skip(self))]
    pub fn run(self, settings: HashMap<String, String>) {
        tracing::debug!("Args: {self:#?}");

        if !self.letters.len() == 12 {
            tracing::error!(
                "String must be exactly 12 letters, {} letters provided.",
                self.letters.len()
            );
            exit(1);
        }

        let letters = self
            .letters
            .chars()
            .map(|l| l.to_ascii_lowercase())
            .collect::<Vec<char>>();

        // Setup settings
        let mut src_directory = settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();
        let mut src_file = settings
            .get("source_file")
            .map_or(DEFAULT_SOURCE_FILE, |v| v)
            .to_string();

        if let Some(sd) = self.dir {
            src_directory = sd;
        };
        if let Some(sf) = self.file {
            src_file = sf;
        };

        let src = format!("{}/{}", src_directory.clone(), src_file.clone());

        let mut words = Vec::new();

        for line in std::fs::read_to_string(&src)
            .expect("Failed to read words file")
            .lines()
        {
            if !line.is_empty() {
                let ws = line.split_whitespace();
                for w in ws {
                    words.push(w.to_string());
                }
            }
        }

        // Get un-shuffled word list
        let mut shuffle = Shuffle::new(true, None, false);
        let mut puzzle = LettersBoxed::new(&letters, &words);
        match puzzle
            .filter_words_with_letters_only()
            .filter_words_with_invalid_pairs()
            .build_word_chain(&mut shuffle)
        {
            Ok(_) => {
                tracing::info!("Word chain built successfully");
            }
            Err(e) => {
                tracing::error!("Failed to build word chain: {}", e);
            }
        };

        let mut solutions = vec![puzzle.solution_string()];

        let mut shuffle = Shuffle::new(false, None, false);
        let mut max_clashes = 10;
        let mut max_solutions = self.random_solutions;

        while max_solutions > 0 && max_clashes > 0 {
            tracing::info!(
                "Generating random solutions max_solutions={max_solutions} and max_clashes={max_clashes}"
            );
            let mut puzzle = LettersBoxed::new(&letters, &words);
            match puzzle
                .filter_words_with_letters_only()
                .filter_words_with_invalid_pairs()
                .build_word_chain(&mut shuffle)
            {
                Ok(_) => {
                    tracing::info!("Word chain built successfully");
                }
                Err(e) => {
                    tracing::error!("Failed to build word chain: {}", e);
                }
            };

            if !solutions.contains(&puzzle.solution_string())
                && puzzle.chain_length() <= self.max_chain
            {
                solutions.push(puzzle.solution_string());
                max_solutions -= 1;
                tracing::debug!(
                    "New solution found: {}, total solutions {}",
                    puzzle.solution_string(),
                    solutions.len()
                );
            } else {
                max_clashes -= 1;
                tracing::debug!(
                    "Clash found: {}, clashes left {}",
                    puzzle.solution_string(),
                    max_clashes
                );
            }
        }

        println!("Solutions:");
        for solution in solutions {
            println!("\t{}", solution);
        }
    }
}
