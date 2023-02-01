use std::process;

mod config;
mod generator;
mod writer;

use config::Config;
use generator::NumberGenerator;

fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(why) => {
            eprintln!("Failed to parse command line arguments: {}", why);
            process::exit(1);
        }
    };
    // println!("{:#?}", new_config);

    let mut generator = NumberGenerator::from_config(&config);

    if let Err(why) =
        writer::write_matrix(&mut generator, &config.output_filename, config.row_count, config.col_count)
    {
        eprintln!("Failed to generate and write matrix: {}", why);
        process::exit(1);
    }
}
