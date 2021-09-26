use std::error::Error;

mod error;
use error::ConfigError;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub row_count: u128,
    pub col_count: u128,

    pub col_delimiter: String,

    pub output_filename: String,

    pub custom_range: bool,
    pub range_from: i128,
    pub range_to: i128,
}

impl Config {
    pub fn new(row_count: u128, col_count: u128) -> Config {
        Config {
            row_count,
            col_count,
            col_delimiter: String::from(" "),
            output_filename: String::from("output.txt"),
            ..Default::default()
        }
    }

    pub fn from_args(args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
        let mandatory_args = &args[1..];
        let mut config = Config::parse_mandatory(mandatory_args)?;

        let optional_args = &args[3..];
        config.parse_optional(optional_args)?;

        Ok(config)
    }

    pub fn parse_mandatory(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            // at least 2 args are required: row_count, col_count
            return Err(Box::from(ConfigError::NotEnoughMandatoryArguments(2, args.len())))
        }

        let row_count = args[0].parse()?;
        let col_count = args[1].parse()?;

        Ok(Config::new(row_count, col_count))
    }

    pub fn parse_optional(&mut self, args: &[String]) -> Result<(), Box<dyn Error>> {
        if args.len() < 1 {
            // no optional parameters
            return Ok(());
        }

        for i in 0..(args.len()) {
            let arg = &args[i];

            if "-d".eq(arg) || "--delimiter".eq(arg) {
                self.col_delimiter = args[i + 1].to_string();
            } else if "-o".eq(arg) || "--output".eq(arg) {
                self.output_filename = args[i + 1].to_string();
            } else if "-r".eq(arg) || "--range".eq(arg) {
                self.range_from = args[i + 1].parse()?;
                self.range_to = args[i + 2].parse()?;
                self.custom_range = true;
            }
        }

        if self.range_to < self.range_from {
            return Err(Box::from(ConfigError::InvalidRangeError(self.range_from, self.range_to)));
        }

        Ok(())
    }

}