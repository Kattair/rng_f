use std::{process, time::Instant};

use clap::Parser;
use colored::*;
use rng_f::{Config, generator_from_config, writer};

fn try_main() -> Result<(), anyhow::Error> {
    let config = Config::parse();
    let mut generator = generator_from_config(&config)?;

    println!("Starting to generate");
    let start_time = Instant::now();

    let common_args = &config.common_args;
    writer::write_matrix_into_file(
        &mut generator,
        &common_args.output_filename,
        common_args.row_count,
        common_args.col_count,
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
