use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No word found")]
    NoWordFound,
    #[error("No word list provided")]
    NoWordList,
    #[error("String must be exactly 9 to 24 letters. Letters Provided: `{}`.", 0)]
    TooFewOrManyLetters(usize),
    #[error("String must be divisible by 3. Letters Provided: `{}`.", 0)]
    MustBeDivisibleBy3(usize),
    #[error("Unknown shape: {}.", 0)]
    UnknownShape(String),
    #[error("Unknown shape for {} edges.", 0)]
    UnknownShapeForEdges(u8),
}
