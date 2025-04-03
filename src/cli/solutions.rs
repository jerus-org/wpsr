use std::collections::HashMap;

use crate::{Error, LettersBoxed, Shape, Shuffle};
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
    #[arg(short, long, default_value_t = 100)]
    pub random_solutions: usize,
    /// maximum length of the word chain
    #[arg(short, long, default_value_t = 10)]
    pub max_chain: usize,
}

impl CmdSolutions {
    #[tracing::instrument(skip(self))]
    pub fn run(self, settings: HashMap<String, String>) -> Result<(), Error> {
        tracing::debug!("Args: {self:#?}");

        if self.letters.len() < 9 || self.letters.len() > 24 {
            return Err(Error::TooFewOrManyLetters(self.letters.len()));
        }

        if !(self.letters.len() % 3) == 0 {
            return Err(Error::MustBeDivisibleBy3(self.letters.len()));
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
        tracing::info!("Using word list: {}", src);

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

        tracing::info!("Get un-shuffled word list");
        let mut shuffle = Shuffle::new(true, None, false);
        let mut puzzle = LettersBoxed::new(&letters, &words);
        match puzzle
            .filter_words_with_letters_only()
            .filter_words_with_invalid_pairs()
            .set_max_chain(self.max_chain)
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
        let mut solution_lengths = HashMap::new();

        let mut shuffle = Shuffle::new(false, None, false);
        let mut max_clashes = 10;
        // let mut max_solutions = self.random_solutions;
        let mut max_solutions = 200;

        while max_solutions > 0 && max_clashes > 0 {
            tracing::info!(
                "Generating random solutions max_solutions={max_solutions} and max_clashes={max_clashes}"
            );
            let mut puzzle = LettersBoxed::new(&letters, &words);
            match puzzle
                .filter_words_with_letters_only()
                .filter_words_with_invalid_pairs()
                .set_max_chain(self.max_chain)
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
            // && puzzle.chain_length() <= self.max_chain
            {
                solutions.push(puzzle.solution_string());
                if let Some(count) = solution_lengths.get(&puzzle.chain_length()) {
                    let v = count + 1;
                    solution_lengths.insert(puzzle.chain_length(), v);
                } else {
                    solution_lengths.insert(puzzle.chain_length(), 1);
                }
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

        tracing::info!("Found {} solutions", solutions.len());

        println!(
            "Solutions for {}:",
            Shape::from_edges((letters.len() / 3) as u8)?
        );

        let mut counts = solution_lengths.iter().collect::<Vec<(&usize, &usize)>>();
        counts.sort_by(|a, b| a.0.cmp(b.0));

        for item in counts {
            println!("  - {:3.0} solutions with {:2.0} words", item.1, item.0);
        }
        for solution in solutions {
            println!("\t{}", solution);
        }

        Ok(())
    }
}
