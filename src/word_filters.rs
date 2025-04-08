//! Prepares a list of words from a file for use in by the solver.

pub trait WordFilters {
    fn filter_to_minimum_length(self, length: usize) -> Self;
    fn filter_no_repeated_letters(self) -> Self;
    fn filter_excludes_letters(self, exclude: &str) -> Self;
    fn filter_includes_only_letters(self, include: &str) -> Self;
    fn filter_includes_any_letters(self, include: &str) -> Self;
    fn filter_includes_all_letters(self, include: &str) -> Self;
}

impl WordFilters for Vec<String> {
    #[tracing::instrument(skip(self))]
    fn filter_to_minimum_length(self, length: usize) -> Self {
        let filtered = self
            .iter()
            .filter(|word| word.len() >= length)
            .map(|word| word.to_string().to_lowercase())
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

    #[tracing::instrument(skip(self))]
    fn filter_excludes_letters(self, exclude: &str) -> Self {
        let excludes = self
            .iter()
            .filter(|word| {
                let chars = word.chars();
                for char in chars {
                    if exclude.contains(char) {
                        return false;
                    }
                }
                true
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            exclude,
            excludes.len()
        );
        excludes
    }

    #[tracing::instrument(skip(self))]
    fn filter_includes_only_letters(self, include: &str) -> Self {
        let includes = self
            .iter()
            .filter(|word| {
                let chars = word.chars();
                for char in chars {
                    if !include.contains(char) {
                        return false;
                    }
                }
                true
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            include,
            includes.len()
        );
        includes
    }

    #[tracing::instrument(skip(self, include))]
    fn filter_includes_all_letters(self, include: &str) -> Self {
        let includes = self
            .iter()
            .filter(|word| {
                let chars = include.chars();
                for char in chars {
                    if !word.contains(char) {
                        return false;
                    }
                }
                true
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            include,
            includes.len()
        );
        includes
    }

    #[tracing::instrument(skip(self, include))]
    fn filter_includes_any_letters(self, include: &str) -> Self {
        let includes = self
            .iter()
            .filter(|word| {
                let chars = word.chars();
                for char in chars {
                    if include.contains(char) {
                        return true;
                    }
                }
                false
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            include,
            includes.len()
        );
        includes
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

    use super::WordFilters;

    #[test]
    fn test_filter_exclude_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.filter_excludes_letters("cl");
        assert_eq!(filtered, vec!["ab", "treat", "greatness"]);
    }

    #[test]
    fn test_filter_include_all_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "alternate",
            "grown",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.filter_includes_all_letters("al");
        assert_eq!(filtered, vec!["all", "alternate"]);
    }

    #[test]
    fn test_filter_include_any_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "grown",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.filter_includes_any_letters("al");
        assert_eq!(
            filtered,
            vec!["ab", "loser", "all", "fulfil", "treat", "greatness",]
        );
    }

    #[test]
    fn test_filter_include_only_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.filter_includes_only_letters("acubsel");
        assert_eq!(filtered, vec!["ab", "all", "success"]);
    }
}
