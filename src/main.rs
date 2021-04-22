use std::env;
use std::error::Error;

#[derive(Debug)]
struct Config {
    row_count: usize,
    col_count: usize,
    spaces: bool,
    output_filename: String,
}

impl Config {
    fn new(row_count: usize, col_count: usize, spaces: bool, output_filename: &str) -> Config {
        let output_filename = String::from(output_filename);

        Config {row_count, col_count, spaces, output_filename}
    }

    /*
        Desired format:
            rng_f rows_count cols_count [OPTION]

            OPTION
            -s, --spaces
                generated space between columns for better user readability
            -o, --output
                specify output_filename, default is output.txt
     */
    fn parse_config(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            panic!("Not enough arguments.");
        }

        let row_count = args[1].parse()?;
        let col_count = args[2].parse()?;
        let mut spaces = false;
        let mut output_filename = String::from("output.txt");

        if args.len() < 4 {
            return Ok(Config {row_count, col_count, spaces, output_filename});
        }

        for i in 3..(args.len()) {
            let arg = &args[i];
            if "-s".eq(arg) || "--spaces".eq(arg) {
                spaces = true;
            }
            else if "-o".eq(arg) || "--output".eq(arg) {
                output_filename = args[i + 1].to_string();
            }
        }

        Ok(Config {row_count, col_count, spaces, output_filename})
    }
}

fn main() {
    let config = Config::parse_config(env::args().collect());
    println!("{:?}", config);
}
