#[derive(Debug, Clone)]
pub struct Edge(char, char, char);

impl Edge {
    pub fn new(first: char, middle: char, last: char) -> Self {
        Self(first, middle, last)
    }

    pub fn pairs(&self) -> Vec<(char, char)> {
        vec![(self.0, self.1), (self.0, self.2), (self.1, self.2)]
    }
}
