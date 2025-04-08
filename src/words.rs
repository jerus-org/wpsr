use std::collections::HashMap;

use colorful::Colorful;

use crate::{DEFAULT_SOURCE_DIR, DEFAULT_WORDS_SOURCE_FILE, Error, WordFilters};

#[derive(Debug, Default)]
pub struct Words {
    settings: HashMap<String, String>,
    letters: Vec<char>,
    word_source: String,
    words: Vec<String>,
    solutions: Vec<String>,
    max: usize,
    required: Option<String>,
    distribution: HashMap<usize, i32>,
}

impl Words {
    pub fn new(letters: &str, settings: HashMap<String, String>) -> Result<Self, Error> {
        if letters.len() < 3 || letters.len() > 26 {
            return Err(Error::TooFewOrManyLetters(letters.len()));
        }

        let letters = letters
            .chars()
            .map(|l| l.to_ascii_lowercase())
            .collect::<Vec<char>>();

        Ok(Self {
            settings,
            letters,
            ..Default::default()
        })
    }

    pub fn set_word_source(&mut self, dir: Option<String>, file: Option<String>) -> &mut Self {
        // Setup settings
        let mut src_directory = self
            .settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();
        let mut src_file = self
            .settings
            .get("source_file")
            .map_or(DEFAULT_WORDS_SOURCE_FILE, |v| v)
            .to_string();

        if let Some(sd) = dir {
            src_directory = sd;
        };
        if let Some(sf) = file {
            src_file = sf;
        };

        let src = format!("{}/{}", src_directory.clone(), src_file.clone());
        println!("Using word list: {}", src);
        tracing::info!("Using word list: {}", src);

        self.word_source = src;

        self
    }

    pub fn load_words(&mut self) -> &mut Self {
        let mut words = Vec::new();

        for line in std::fs::read_to_string(&self.word_source)
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

    pub fn set_max_solutions(&mut self, max: usize) -> &mut Self {
        self.max = max;
        self
    }

    pub fn set_required(&mut self, required: Option<String>) -> &mut Self {
        self.required = required;
        self
    }

    #[tracing::instrument(skip(self))]
    pub fn find_solutions(&mut self) -> Result<&mut Self, Error> {
        tracing::info!("Get un-shuffled word list");
        let all_letters = "abcdefghijklmnopqrstuvwxyz";
        let excluded_letters = all_letters
            .chars()
            .filter(|&c| !self.letters.contains(&c))
            .collect::<String>();

        let words = self.words.clone();
        println!("{} words found", words.len());
        let mut filtered = words.filter_excludes_letters(&excluded_letters);
        if let Some(required) = &self.required {
            filtered = filtered.filter_includes_any_letters(required);
        }
        println!("{} words found", filtered.len());

        filtered.sort_by(|a, b| {
            let a_len = a.len();
            let b_len = b.len();
            b_len.cmp(&a_len)
        });

        // let mut final_list = Vec::new();

        // for (i, word) in filtered.iter().enumerate() {
        //     if i >= self.max {
        //         break;
        //     }

        //     self.count_solution(word.len());
        //     final_list.push(word.clone());
        // }

        let final_list = filtered
            .iter()
            .take(self.max)
            .cloned()
            .inspect(|w| {
                self.count_solution(w.len());
            })
            .collect::<Vec<String>>();

        self.solutions = final_list;

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

    pub fn word_source_string(&self) -> String {
        let s1 = "Using words sourced from ".light_cyan().dim().to_string();
        let s2 = self.word_source.clone().light_cyan().bold().to_string();
        format!("{}{}", s1, s2)
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

    pub fn solutions_title(&self) -> String {
        let intro = "Words using the letters ";
        let mut ul = String::new();
        for _ in 0..(intro.len() + self.letters.len()) {
            ul.push('‾');
        }

        let summary = format!(
            "{}{}",
            intro.yellow().bold(),
            self.letters.iter().collect::<String>().blue().bold()
        );
        format!("{}\n{}", summary, ul.bold().yellow())
    }

    pub fn solutions_string(&self) -> String {
        let mut s = String::new();
        let mut solutions = self
            .solutions
            .iter()
            .map(|s| (s.len(), s))
            .collect::<Vec<_>>();
        solutions.sort_by(|a, b| a.0.cmp(&b.0));

        let mut word_length = solutions.first().unwrap_or(&(0, &"".to_string())).0;

        s.push_str("  ");
        s.push_str(
            &format!(
                "{} Solutions with {} words.",
                self.distribution.get(&word_length).unwrap_or(&0),
                word_length
            )
            .underlined()
            .yellow()
            .to_string(),
        );
        s.push_str("\n\n");

        for solution in solutions {
            if solution.0 != word_length {
                word_length = solution.0;
                s.push_str("\n  ");
                s.push_str(
                    &format!(
                        "{} Solutions with {} words.",
                        self.distribution.get(&word_length).unwrap_or(&0),
                        word_length
                    )
                    .underlined()
                    .yellow()
                    .to_string(),
                );
                s.push_str("\n\n");
            }
            s.push_str(&format!("    {}\n", solution.1));
        }
        s
    }
}
