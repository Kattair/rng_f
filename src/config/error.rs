use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ConfigError {
    InvalidRangeError(i64, i64),
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ConfigError::InvalidRangeError(range_from, range_to) => write!(f, "Invalid range: {} < {}", range_to, range_from)?,
        };

        Ok(())
    }
}