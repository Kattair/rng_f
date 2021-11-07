use std::error::Error;
use std::process;

use config::Config;
use generator::NumberGenerator;
use writer::Writer;

mod config;
mod generator;
mod writer;

fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(why) => {
            eprintln!("Failed to parse command line arguments: {}", why);
            process::exit(1);
        }
    };
    // println!("{:#?}", new_config);

    let generator = NumberGenerator::from_config(&config);
    let mut writer = Writer::new(generator);

    if let Err(why) =
        writer.write_matrix(&config.output_filename, config.row_count, config.col_count)
    {
        eprintln!("Failed to generate and write matrix: {}", why);
        process::exit(1);
    }
}
