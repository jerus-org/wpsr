use std::{collections::VecDeque, fmt::Display};

mod edge;
mod shuffle;
mod weighted_word;

use edge::Edge;
use rand::{SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha20Rng;
pub use shuffle::Shuffle;
use weighted_word::WeightedWord;

#[derive(Debug)]
pub struct LettersBoxed {
    letters: Vec<char>,
    words: Vec<String>,
    invalid_pairs: Vec<(char, char)>,
    word_chain: Vec<String>,
    edges: Vec<Edge>,
    max_chain: Option<usize>,
}

impl Default for LettersBoxed {
    fn default() -> Self {
        let letters = vec!['o', 'u', 'h', 'i', 'm', 'a', 'g', 'p', 'l', 'r', 'y', 'f'];
        let edges = vec![
            Edge::new('o', 'u', 'h'),
            Edge::new('i', 'm', 'a'),
            Edge::new('g', 'p', 'l'),
            Edge::new('r', 'y', 'f'),
        ];
        Self {
            letters,
            words: Vec::new(),
            invalid_pairs: Vec::new(),
            word_chain: Vec::new(),
            edges,
            max_chain: None,
        }
    }
}

impl LettersBoxed {
    pub fn new(letters: &[char], words: &[String]) -> Self {
        let mut s = Self::default();
        if !letters.is_empty() {
            s.letters = Vec::from(letters);
            s.generate_edges();
        }
        if !words.is_empty() {
            s.words = Vec::from(words);
        }

        s
    }

    pub fn set_max_chain(&mut self, max_chain: usize) -> &mut Self {
        self.max_chain = Some(max_chain);
        self
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
    fn generate_edges(&mut self) -> &mut Self {
        let mut deque = VecDeque::from(self.letters.clone());

        while !deque.is_empty() {
            let edge = Edge::new(
                deque.pop_front().unwrap(),
                deque.pop_front().unwrap(),
                deque.pop_front().unwrap(),
            );
            self.edges.push(edge);
        }

        self
    }

    #[tracing::instrument(skip(self))]
    fn generate_invalid_pairs(&mut self) -> &mut Self {
        let mut invalid_pairs = Vec::new();
        for edge in self.edges.iter() {
            for pair in edge.pairs() {
                if !self.invalid_pairs.contains(&pair) {
                    invalid_pairs.push(pair);
                }
            }
        }

        tracing::debug!("Generated {} invalid pairs", invalid_pairs.len());
        tracing::trace!("Invalid pairs: {:#?}", invalid_pairs);
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
                    if self.invalid_pairs.contains(&(a, *b))
                        || self.invalid_pairs.contains(&(*b, a))
                    {
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

    #[tracing::instrument(skip(self, shuffle))]
    pub fn build_word_chain(&mut self, shuffle: &mut Shuffle) -> Result<(), Error> {
        tracing::info!("Building word chain");
        // Get the first word from the list of words
        let mut rng = ChaCha20Rng::from_os_rng();
        let words = self.words.clone();
        let word_list = words.clone();
        let word_chain = Vec::new();
        let unused_letters = String::from_iter(self.letters.clone());

        let word_chain = get_word(
            words,
            word_list,
            word_chain,
            unused_letters,
            &mut rng,
            shuffle,
            self.max_chain,
        )?;

        self.word_chain = word_chain;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn solution_string(&self) -> String {
        self.word_chain.join(" -> ").to_string()
    }

    #[tracing::instrument(skip(self))]
    pub fn chain_length(&self) -> usize {
        self.word_chain.len()
    }
}

#[tracing::instrument(skip(
    all_words,
    words_list,
    word_chain,
    unused_letters,
    rng,
    shuffle,
    max_chain
))]
pub fn get_word(
    all_words: Vec<String>,
    mut words_list: Vec<String>,
    mut word_chain: Vec<String>,
    mut unused_letters: String,
    rng: &mut ChaCha20Rng,
    shuffle: &mut Shuffle,
    max_chain: Option<usize>,
) -> Result<Vec<String>, Error> {
    let initial_unused_letters = unused_letters.clone();
    tracing::trace!("Starting word chain: {}", word_chain.join(" -> "));
    tracing::trace!(
        "Shuffle set to: {} and word list length: {}",
        shuffle,
        words_list.len()
    );

    // Shuffle the starting words list to get a random starting word
    if shuffle == &Shuffle::Twice {
        tracing::debug!("Shuffling words list.");
        words_list.shuffle(rng);
    }
    let mut words_list = words_list
        .iter()
        .map(|word| WeightedWord::new(word.clone(), unused_letters.clone()))
        .collect::<Vec<WeightedWord>>();
    words_list.sort_by_key(|ww| ww.weight);
    let words_list = words_list
        .iter()
        .rev()
        .map(|ww| ww.word.to_string())
        .collect::<Vec<String>>();

    // shuffle the top of to the words list to randomize the first word while keeping a good weight
    let mut words = if shuffle != &Shuffle::None {
        tracing::debug!("Shuffling top half of weighted words list.");
        let words_list = shuffle_top_half(words_list, rng);
        VecDeque::from(words_list)
    } else {
        VecDeque::from(words_list)
    };

    // find a word that increases the letters used

    loop {
        tracing::trace!(
            "List of {} words starting with: {:?}",
            words.len(),
            words.front()
        );
        let Some(word) = words.pop_front() else {
            return Err(Error::NoWordFound);
        };

        let letter_count = &unused_letters.len();
        tracing::trace!("Letters unused before check: {}", unused_letters);
        for letter in word.chars() {
            if let Some(idx) = unused_letters.find(letter) {
                unused_letters.remove(idx);
            }
        }
        tracing::trace!("Letters unused after check: {}", unused_letters);

        // if all of the letters are used then we have the final word chain
        if unused_letters.is_empty() {
            word_chain.push(word);
            break;
        }
        tracing::trace!(
            "Still {} letters unused: {}",
            unused_letters.len(),
            unused_letters
        );

        // if the word extends the chain add it to the chain and start again
        if unused_letters.len() < *letter_count {
            if let Some(max_chain) = max_chain {
                if word_chain.len() >= max_chain {
                    return Err(Error::ChainTooLong);
                }
            }
            let mut next_word_chain = word_chain.clone();
            let next_unused_letters = unused_letters.clone();
            let next_all_words = all_words.clone();
            let last_letter = word.chars().last().unwrap();
            let words_list = all_words
                .iter()
                .filter(|w| w.chars().next().unwrap() == last_letter)
                .map(|w| w.to_string())
                .collect::<Vec<String>>();
            if words.is_empty() {
                return Err(Error::NoWordFound);
            }
            next_word_chain.push(word);
            match get_word(
                next_all_words,
                words_list,
                next_word_chain,
                next_unused_letters,
                rng,
                shuffle,
                max_chain,
            ) {
                Ok(chain) => {
                    word_chain = chain;
                    break;
                }
                Err(e) => {
                    tracing::debug!("No word found, resetting");
                    if e == Error::ChainTooLong {
                        return Err(e);
                    }
                    unused_letters = initial_unused_letters.clone();
                    continue;
                }
            };
        }
    }

    tracing::debug!("Current word chain: {}", word_chain.join("-"));

    Ok(word_chain)
}

fn shuffle_top_half(mut words: Vec<String>, rng: &mut ChaCha20Rng) -> Vec<String> {
    let half_len = words.len() / 2;
    let mut top_half = words.drain(..half_len).collect::<Vec<String>>();
    let bottom_half = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();
    top_half.shuffle(rng);
    top_half.extend(bottom_half);
    top_half
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NoWordFound,
    ChainTooLong,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoWordFound => write!(f, "No word found"),
            Error::ChainTooLong => write!(f, "Chain too long"),
        }
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

        let mut letters_boxed = LettersBoxed::new(&letters, &words);
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

        let mut letters_boxed = LettersBoxed::new(&letters, &words);

        letters_boxed.filter_words_with_letters_only();
        println!("{:#?}", letters_boxed.words);

        letters_boxed.filter_words_with_invalid_pairs();
        println!("{:#?}", letters_boxed.words);

        assert_eq!(letters_boxed.words.len(), 3);
        assert_eq!(letters_boxed.words[0], "world".to_string());
        assert_eq!(letters_boxed.words[1], "game".to_string());
        assert_eq!(letters_boxed.words[2], "waldo".to_string());
    }

    #[test]
    fn test_shuffle_top_half() {
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ];
        println!("before: {:?}", words);
        let mut rng = ChaCha20Rng::seed_from_u64(1);
        let shuffled = shuffle_top_half(words, &mut rng);
        println!("after: {:?}", shuffled);
        assert_eq!(shuffled.len(), 5);
        assert_eq!(shuffled[0], "world".to_string());
        assert_eq!(shuffled[1], "hello".to_string());
        assert_eq!(shuffled[2], "foo".to_string());
        assert_eq!(shuffled[3], "bar".to_string());
        assert_eq!(shuffled[4], "baz".to_string());
    }
}
