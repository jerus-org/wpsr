use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No word found")]
    NoWordFound,
    #[error("No word list provided")]
    NoWordList,
    #[error("String must be exactly 12 letters, {} letters provided.", 0)]
    MustBe12Letters(usize),
}
