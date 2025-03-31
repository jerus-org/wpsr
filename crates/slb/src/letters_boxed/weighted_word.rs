use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WeightedWord {
    pub word: String,
    pub weight: usize,
}

impl WeightedWord {
    pub fn new(word: String, unused_letters: String) -> Self {
        let set: HashSet<char> = word.chars().collect();
        let weight = unused_letters.chars().filter(|c| set.contains(c)).count();
        WeightedWord { word, weight }
    }
}
