//! Prepares a list of words from a file for use in by the solver.

pub trait PrepareWords {
    fn filter_to_minimum_length(self, length: usize) -> Self;
    fn filter_no_repeated_letters(self) -> Self;
}

impl PrepareWords for Vec<String> {
    #[tracing::instrument(skip(self))]
    fn filter_to_minimum_length(self, length: usize) -> Self {
        let filtered = self
            .iter()
            .filter(|word| word.len() >= length)
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!("Filtered to {} words", filtered.len());
        filtered
    }

    #[tracing::instrument(skip(self))]
    fn filter_no_repeated_letters(self) -> Self {
        let no_repeated_letters = self
            .iter()
            .filter(|word| {
                let mut chars = word.chars();
                let mut a = chars.next().unwrap();
                for b in chars {
                    if a == b {
                        return false;
                    }
                    a = b
                }
                true
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no repeated letters there are {} words",
            no_repeated_letters.len()
        );
        no_repeated_letters
    }
}

#[allow(dead_code)]
fn no_repeated_letter(word: &str) -> bool {
    let mut chars = word.chars();
    let mut a = chars.next().unwrap();
    for b in chars {
        if a == b {
            return false;
        }
        a = b
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_repeated_letter() {
        let words = vec!["ab", "aa", "all", "success", "fulfil", "greatness"];

        let mut passed = vec![];
        let mut failed = vec![];
        for word in words {
            if no_repeated_letter(word) {
                passed.push(word);
            } else {
                failed.push(word);
            }
        }
        assert_eq!(passed, vec!["ab", "fulfil"]);
        assert_eq!(failed, vec!["aa", "all", "success", "greatness"]);
    }
}
