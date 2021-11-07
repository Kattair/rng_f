use clap::Parser;
use std::error::Error;

use super::Config;
use super::ConfigError;

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let config = Config::parse();

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // need to validate range
        self.validate_range()?;

        Ok(())
    }

    fn validate_range(&self) -> Result<(), ConfigError> {
        if let Some(range) = &self.range {
            let from = range[0];
            let to = range[1];

            if from > to {
                return Err(ConfigError::InvalidRangeError(from, to));
            }
        }

        Ok(())
    }
}
