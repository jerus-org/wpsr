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

    #[tracing::instrument(skip(self))]
    pub fn filter_words_with_letters_only(&mut self) -> &mut Self {

        let filtered = self
            .words
        .iter()
        .filter(|word| word.chars().all(|c| self.letters.contains(&c)))
        .map(|w| w.to_string())
        .collect::<Vec<String>>();

        tracing::info!("Filtered to {} words", filtered.len());
        self.words = filtered;
        self
    }
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_filter_words_with_letters_only() {
        let letters = vec!['o', 'u', 'h', 'i', 'd', 'a', 'g', 'e', 'l', 'r', 'y', 'w'];
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ];

        let mut letters_boxed = LettersBoxed::new(letters, words);
        letters_boxed.filter_words_with_letters_only();
        assert_eq!(letters_boxed.words.len(), 2);
        assert_eq!(letters_boxed.words[0], "hello".to_string());
        assert_eq!(letters_boxed.words[1], "world".to_string());

    }
}
