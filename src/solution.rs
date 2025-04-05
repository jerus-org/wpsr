use std::collections::HashMap;

use crate::{Error, LettersBoxed, Shape, Shuffle};

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "default.slb";

#[derive(Debug, Default)]
pub struct Solution {
    settings: HashMap<String, String>,
    letters: Vec<char>,
    words: Vec<String>,
    max_chain: usize,
    shuffle_depth: i8,
    solutions: Vec<String>,
    distribution: HashMap<usize, i32>,
}

impl Solution {
    pub fn new(letters: &str, settings: HashMap<String, String>) -> Result<Self, Error> {
        if letters.len() < 9 || letters.len() > 24 {
            return Err(Error::TooFewOrManyLetters(letters.len()));
        }

        if !(letters.len() % 3) == 0 {
            return Err(Error::MustBeDivisibleBy3(letters.len()));
        }

        let letters = letters
            .chars()
            .map(|l| l.to_ascii_lowercase())
            .collect::<Vec<char>>();

        Ok(Self {
            settings,
            letters,
            max_chain: 10,
            shuffle_depth: 3,
            ..Default::default()
        })
    }

    pub fn load_words(&mut self, dir: Option<String>, file: Option<String>) -> &mut Self {
        // Setup settings
        let mut src_directory = self
            .settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();
        let mut src_file = self
            .settings
            .get("source_file")
            .map_or(DEFAULT_SOURCE_FILE, |v| v)
            .to_string();

        if let Some(sd) = dir {
            src_directory = sd;
        };
        if let Some(sf) = file {
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

        self.words = words;

        self
    }

    pub fn set_max_chain(&mut self, value: usize) -> &mut Self {
        self.max_chain = value;
        self
    }

    pub fn set_shuffle_depth(&mut self, value: i8) -> &mut Self {
        self.shuffle_depth = value;
        self
    }

    pub fn find_best_solution(&mut self) -> Result<&mut Self, Error> {
        tracing::info!("Get un-shuffled word list");
        let mut shuffle = Shuffle::None;
        let mut puzzle = LettersBoxed::new(&self.letters, &self.words);
        match puzzle
            .filter_words_with_letters_only()
            .filter_exclude_invalid_pairs()
            .set_max_chain(self.max_chain)
            .build_word_chain(&mut shuffle)
        {
            Ok(_) => {
                tracing::info!("Word chain built successfully");
                self.solutions.push(puzzle.solution_string());
                self.count_solution(puzzle.chain_length());
            }
            Err(e) => {
                tracing::error!("Failed to build word chain: {}", e);
            }
        };

        Ok(self)
    }

    #[tracing::instrument(skip(self))]
    pub fn find_random_solution(&mut self, mut shuffle: Shuffle) -> Result<&mut Self, Error> {
        tracing::info!("Get un-shuffled word list");
        let mut puzzle = LettersBoxed::new(&self.letters, &self.words);
        match puzzle
            .filter_words_with_letters_only()
            .filter_exclude_invalid_pairs()
            .set_max_chain(self.max_chain)
            .set_shuffle_depth(self.shuffle_depth)
            .build_word_chain(&mut shuffle)
        {
            Ok(_) => {
                tracing::info!("Word chain built successfully");
                if self.solutions.contains(&puzzle.solution_string()) {
                    tracing::info!("Solution already found");
                    return Err(Error::SolutionAlreadyFound);
                }
                self.solutions.push(puzzle.solution_string());
                self.count_solution(puzzle.chain_length());
            }
            Err(e) => {
                tracing::error!("Failed to build word chain: {}", e);
                return Err(e);
            }
        };

        Ok(self)
    }

    pub fn count_solution(&mut self, chain_length: usize) -> &mut Self {
        if let Some(count) = self.distribution.get(&chain_length) {
            let v = count + 1;
            self.distribution.insert(chain_length, v);
        } else {
            self.distribution.insert(chain_length, 1);
        }

        self
    }

    pub fn shape_string(&self) -> String {
        match Shape::from_edges((self.letters.len() / 3) as u8) {
            Ok(shape) => shape.to_string(),
            Err(_) => "Unknown shape".to_string(),
        }
    }

    pub fn distribution_string(&self) -> String {
        let mut s = String::new();
        let mut distributions = self.distribution.iter().collect::<Vec<_>>();
        distributions.sort_by(|a, b| a.0.cmp(b.0));

        for d in distributions {
            s.push_str(&format!(
                "  - {:3.0} solutions with {:2.0} words\n",
                d.1, d.0
            ));
        }
        s
    }

    pub fn solutions_string(&self) -> String {
        let mut s = String::new();
        let mut solutions = self
            .solutions
            .iter()
            .map(|s| {
                let words = s.chars().filter(|c| *c == '>').count() + 1;
                (words, s)
            })
            .collect::<Vec<_>>();
        solutions.sort_by(|a, b| a.0.cmp(&b.0));

        let mut word_length = solutions.first().unwrap_or(&(0, &"".to_string())).0;

        s.push_str(&format!(
            "  {} Solutions with {} words.\n\n",
            self.distribution.get(&word_length).unwrap_or(&0),
            word_length
        ));

        for solution in solutions {
            if solution.0 != word_length {
                word_length = solution.0;
                s.push_str(&format!(
                    "\n  {} Solutions with {} words.\n\n",
                    self.distribution.get(&word_length).unwrap_or(&0),
                    word_length
                ));
            }
            s.push_str(&format!("    {}\n", solution.1));
        }
        s
    }
}
