#[derive(Debug)]
pub struct LettersBoxed {
    pub letters: Vec<char>,
    pub words: Vec<String>,
}

impl Default for LettersBoxed {
    fn default() -> Self {
        let letters = vec!['o', 'u', 'h', 'i', 'm', 'a', 'g', 'p', 'l', 'r', 'y', 'f'];
        Self {
            letters,
            words: Vec::new(),
        }
    }
}

impl LettersBoxed {
    pub fn new(letters: Vec<char>, words: Vec<String>) -> Self {
        let mut s = Self::default();
        if !letters.is_empty() {
            s.letters = letters;
        }
        if !words.is_empty() {
            s.words = words;
        }

        s
    }
}
