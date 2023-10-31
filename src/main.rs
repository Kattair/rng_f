use std::{error::Error, process, time::Instant};

use clap::Parser;
use colored::*;
use rng_f::{config::Config, generator::NumberGenerator, writer};

fn try_main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let mut generator = NumberGenerator::new(config.range(), &config.delimiter)?;

    println!("Starting generation");
    let start_time = Instant::now();

    writer::write_matrix_into_file(
        &mut generator,
        &config.output_filename,
        config.row_count,
        config.col_count,
    )?;

    let elapsed_time = start_time.elapsed().as_millis();
    println!("Generation took {} ms to complete", elapsed_time);

    Ok(())
}

fn main() {
    if let Err(why) = try_main() {
        eprintln!("{} {}", "error:".bold().red(), why);
        process::exit(1);
    }
}
