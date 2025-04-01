use std::{collections::HashMap, process::exit};

use clap::Parser;
use slb::{LettersBoxed, Shuffle};

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "mit_words.slb";

#[derive(Parser, Debug, Clone)]
pub struct CmdSolutions {
    pub letters: Vec<char>,
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
    /// word list source file
    #[arg(short, long)]
    pub file: Option<String>,
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
    pub fn run(self, settings: HashMap<String, String>) {
        tracing::debug!("Args: {self:#?}");

        let mut letters = Vec::new();

        if !self.letters.len() == 12 {
            tracing::error!(
                "Must supply exactly 12 letters, {} letters provided.",
                self.letters.len()
            );
            exit(1);
        }

        if !self.letters.is_empty() {
            letters = self
                .letters
                .iter()
                .map(|l| l.to_ascii_lowercase())
                .collect::<Vec<char>>();
        }
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

        let mut shuffle = Shuffle::new(false, None, false);
        let mut puzzle = LettersBoxed::new(letters, words);
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

        println!("Word chain: {}", puzzle.solution_string());
    }
}
