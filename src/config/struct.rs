use clap::{crate_name, Parser};

#[derive(Parser, Clone, Debug)]
#[clap(name = crate_name!(), author, about, version)]
pub struct Config {
    /// Specify the number of rows
    #[clap(required = true)]
    pub row_count: u128,
    /// Specify the number of columns
    #[clap(required = true)]
    pub col_count: u128,

    /// Define a string to be used as the delimiter between columns
    #[clap(short, long, default_value = " ", display_order = 1)]
    pub delimiter: String,

    /// Specify output filename
    #[clap(
        short,
        long,
        value_name = "FILENAME",
        default_value = "output.txt",
        display_order = 2
    )]
    pub output_filename: String,

    /// Specify bottom and top limit for number range. Creates interval <from, to)
    #[clap(short, long, allow_hyphen_values = true, value_names = &["FROM", "TO"], display_order = 3)]
    pub range: Option<Vec<i128>>,
}
