use std::ops::Range;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(about, version)]
pub struct Config {
    #[command(flatten)]
    pub common_args: CommonGeneratorArgs,

    #[command(subcommand)]
    pub generator: Generators,
}

#[derive(Args, Clone, Debug)]
pub struct CommonGeneratorArgs {
    /// Specify the number of rows
    #[arg(display_order = 1)]
    pub row_count: u128,
    /// Specify the number of columns
    #[arg(display_order = 2)]
    pub col_count: u128,
    /// Define a string to be used as the delimiter between columns
    #[arg(short, long, default_value = " ", display_order = 3)]
    pub delimiter: String,
    /// Specify output filename
    #[arg(
        short,
        long,
        value_name = "FILENAME",
        default_value = "output.txt",
        display_order = 4
    )]
    pub output_filename: String,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Generators {
    /// Generates matrices consisting of integers
    Number(NumberGeneratorArgs),
    /// Generates matrices consisting of non-whitespace ASCII characters
    Ascii(AsciiGeneratorArgs),
}

#[derive(Args, Clone, Debug)]
pub struct NumberGeneratorArgs {
    /// Shorthand to specify bottom and top limit for number range - creates interval <from, to)
    #[arg(
        short,
        long,
        num_args = 2,
        allow_hyphen_values = true,
        value_names = &["FROM", "TO"],
        display_order = 1,
        conflicts_with_all = &["range_from", "range_to"])]
    range: Option<Vec<i64>>,

    /// Specify bottom limit for number range (inclusive)
    #[arg(
        short = 'f',
        long,
        value_name = "FROM",
        allow_hyphen_values = true,
        default_value(i64::MIN.to_string()),
        display_order = 2
    )]
    range_from: i64,

    /// Specify upper limit for number range (non-inclusive)
    #[arg(
        short = 't',
        long,
        value_name = "TO",
        allow_hyphen_values = true,
        default_value = i64::MAX.to_string(),
        display_order = 3
    )]
    range_to: i64,
}

impl NumberGeneratorArgs {
    pub fn range(&self) -> Range<i64> {
        match &self.range {
            Some(range) => range[0]..range[1],
            None => self.range_from..self.range_to,
        }
    }
}

#[derive(Args, Clone, Debug)]
pub struct AsciiGeneratorArgs {
    /// Specify list of ascii characters to use
    #[arg(short = 'l', long, default_value = "abcABC", display_order = 1)]
    pub list: String,
}
