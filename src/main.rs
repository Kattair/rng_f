use std::{env, process};

use config::Config;
use generator::Generator;
use writer::Writer;

mod config;
mod generator;
mod writer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::from_args(&args) {
        Ok(config) => config,
        Err(why) => {
            println!("Failed to parse command line arguments: {}", why);
            process::exit(1);
        }
    };
    // println!("{:?}", config);

    let generator = Generator::from_config(config.clone());
    let mut writer = Writer::new(generator);

    if let Err(why) =
        writer.write_matrix(&config.output_filename, config.row_count, config.col_count)
    {
        println!("Failed to generate and write matrix: {}", why);
        process::exit(1);
    }
}
