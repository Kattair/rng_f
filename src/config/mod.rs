use std::error::Error;

use crate::config::error::ConfigError;

pub mod error;

#[derive(Debug)]
pub struct Config {
    pub row_count: usize,
    pub col_count: usize,
    pub spaces: bool,
    pub output_filename: String,
    pub custom_range: bool,
    pub range_from: i64,
    pub range_to: i64,
}

impl Config {
    fn new(row_count: usize, col_count: usize, spaces: bool, output_filename: &str) -> Config {
        let output_filename = String::from(output_filename);
        let custom_range = false;
        let range_from = 0;
        let range_to = 0;

        Config { row_count, col_count, spaces, output_filename, custom_range, range_from, range_to }
    }

    fn new_with_range(row_count: usize, col_count: usize, spaces: bool, output_filename: &str, custom_range: bool, range_from: i64, range_to: i64) -> Config {
        let output_filename = String::from(output_filename);

        Config { row_count, col_count, spaces, output_filename, custom_range, range_from, range_to }
    }

    /*
        Desired format:
            rng_f rows_count cols_count [OPTION]

            OPTION
            -s, --spaces
                generated space between columns for better user readability
            -o, --output
                specify output_filename, default is output.txt
            -r, --range
                specify bottom limit and top limit for number range
     */
    pub fn parse_config(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            // at least 3 are required: program name, row_count, col_count
            panic!("Not enough arguments.");
        }

        let row_count = args[1].parse()?;
        let col_count = args[2].parse()?;
        let mut spaces = false;
        let mut output_filename = String::from("output.txt");
        let mut custom_range = false;
        let mut range_from = 0;
        let mut range_to = 0;

        if args.len() < 4 {
            // no optional parameters
            return Ok(Config::new(row_count, col_count, spaces, &output_filename));
        }

        for i in 3..(args.len()) {
            let arg = &args[i];
            if "-s".eq(arg) || "--spaces".eq(arg) {
                spaces = true;
            } else if "-o".eq(arg) || "--output".eq(arg) {
                output_filename = args[i + 1].to_string();
            } else if "-r".eq(arg) || "--range".eq(arg) {
                range_from = args[i + 1].parse()?;
                range_to = args[i + 2].parse()?;
                custom_range = true;
            }
        }

        if range_to < range_from {
            return Err(Box::from(ConfigError::InvalidRangeError(range_from, range_to)));
        }

        Ok(Config::new_with_range(row_count, col_count, spaces, &output_filename, custom_range, range_from, range_to))
    }
}