use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Not enough arguments to create range. Argument count: {0}")]
    NotEnoughArgumentsToCreateRangeError(usize),
    #[error("Invalid range: {to:?} < {from:?}")]
    InvalidRangeError { from: i128, to: i128 },
}
