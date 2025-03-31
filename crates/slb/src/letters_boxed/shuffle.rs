#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shuffle {
    no_shuffle: bool,
    shuffles: Option<usize>,
    twice: bool,
}

impl Shuffle {
    pub fn new(no_shuffle: bool, shuffles: Option<usize>, twice: bool) -> Self {
        Self {
            no_shuffle,
            shuffles,
            twice,
        }
    }

    pub fn no_shuffle(&self) -> bool {
        self.no_shuffle
    }

    pub fn twice(&self) -> bool {
        self.twice
    }

    pub fn shuffles(&self) -> Option<usize> {
        self.shuffles
    }

    pub fn shuffles_value(&self) -> usize {
        self.shuffles.unwrap_or(1)
    }

    pub fn decrement_shuffles(&mut self) {
        if let Some(s) = self.shuffles {
            if s > 0 {
                self.shuffles = Some(s - 1);
            }
        }
    }

    pub fn shuffle_words(&self) -> bool {
        !self.no_shuffle && self.shuffles_value() > 0 && self.twice
    }

    pub fn shuffle_weighted(&self) -> bool {
        !self.no_shuffle && self.shuffles_value() > 0
    }
}
