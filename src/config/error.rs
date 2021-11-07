use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ConfigError {
    InvalidRangeError(i128, i128),
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ConfigError::InvalidRangeError(from, to) => {
                write!(f, "Invalid range: {} < {}", to, from)?
            }
        };

        Ok(())
    }
}
