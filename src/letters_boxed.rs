#[derive(Debug)]
pub struct LettersBoxed {
    letters: Vec<char>,
    words: Vec<String>,
    invalid_pairs: Vec<(char, char)>,
}

impl Default for LettersBoxed {
    fn default() -> Self {
        let letters = vec!['o', 'u', 'h', 'i', 'm', 'a', 'g', 'p', 'l', 'r', 'y', 'f'];
        Self {
            letters,
            words: Vec::new(),
            invalid_pairs: Vec::new(),
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

    #[tracing::instrument(skip(self))]
    fn generate_invalid_pairs(&mut self) -> &mut Self {
        let invalid_pairs = vec![
            (self.letters[0], self.letters[1]),
            (self.letters[1], self.letters[2]),
            (self.letters[0], self.letters[2]),
            
            (self.letters[3], self.letters[4]),
            (self.letters[4], self.letters[5]),
            (self.letters[3], self.letters[5]),

            (self.letters[6], self.letters[7]),
            (self.letters[7], self.letters[8]),
            (self.letters[6], self.letters[8]),

            (self.letters[9], self.letters[10]),
            (self.letters[10], self.letters[11]),
            (self.letters[9], self.letters[11]),
        ];

        tracing::info!("Generated {} invalid pairs", invalid_pairs.len());
        tracing::debug!("Invalid pairs: {:#?}", invalid_pairs);
        self.invalid_pairs = invalid_pairs;

        self
    }

    #[tracing::instrument(skip(self))]
    pub fn filter_words_with_invalid_pairs(&mut self) -> &mut Self {
        self.generate_invalid_pairs();

        let filtered = self
            .words
            .iter()
            .filter(|word| {
                let chars = word.chars().collect::<Vec<char>>();
                let mut a = chars[0];
                for b in chars.iter().skip(1) {
                    if self.invalid_pairs.contains(&(a, *b)) || self.invalid_pairs.contains(&(*b, a)) {
                        return false;
                    }
                    a = *b;
                }
                true
            })
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

    #[test]
    fn test_filter_words_with_invalid_pairs() {
        let letters = vec!['o', 'u', 'm', 'i', 'd', 'a', 'g', 'e', 'l', 'r', 'y', 'w'];
        let words = vec![
            "embolden".to_string(),
            "world".to_string(),
            "foo".to_string(),
            "dire".to_string(),
            "glow".to_string(),
            "game".to_string(),
            "quux".to_string(),
            "corse".to_string(),
            "gaunt".to_string(),
            "grapey".to_string(),
            "waldo".to_string(),
            "fred".to_string(),
        ];

        let mut letters_boxed = LettersBoxed::new(letters, words);

        letters_boxed.filter_words_with_letters_only();
        println!("{:#?}", letters_boxed.words);

        letters_boxed.filter_words_with_invalid_pairs();
        println!("{:#?}", letters_boxed.words);

        assert_eq!(letters_boxed.words.len(), 3);
        assert_eq!(letters_boxed.words[0], "world".to_string());
        assert_eq!(letters_boxed.words[1], "game".to_string());
        assert_eq!(letters_boxed.words[2], "waldo".to_string());
    }
}
