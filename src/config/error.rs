use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ConfigError {
    NotEnoughMandatoryArguments(usize, usize),
    InvalidRangeError(i128, i128),
    _MissingValueForFlag(String),
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ConfigError::NotEnoughMandatoryArguments(expected, actual) => write!(f, "Not enough mandatory arguments. Expected {}, received {}", expected, actual)?,
            ConfigError::InvalidRangeError(range_from, range_to) => write!(f, "Invalid range: {} < {}", range_to, range_from)?,
            ConfigError::_MissingValueForFlag(flag) => write!(f, "Missing value or values for flag {}", flag)?,
        };

        Ok(())
    }
}