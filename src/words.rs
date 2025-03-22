//! Prepares a list of words from a file for use in by the solver.

#[derive(Debug, Default)]
pub struct Words {
    words: Vec<String>,
}

impl Words {
    pub fn new(path: &str) -> Self {
        let words: Vec<String> = std::fs::read_to_string(path)
            .expect("Failed to read words file")
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        log::info!("Loaded {} words", words.len());
        Self { words }
    }
}
