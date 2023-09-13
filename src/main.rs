use std::time::Instant;

use rng_f::{config::Config, generator::NumberGenerator, writer};

fn main() {
    let config = Config::new().expect("Failed to parse command line arguments");
    let range = match config.range {
        Some(range) => range[0]..range[1],
        None => i64::MIN..i64::MAX,
    };
    let mut generator = NumberGenerator::new(range, &config.delimiter);

    println!("Starting generation");
    let start_time = Instant::now();

    writer::write_matrix_into_file(
        &mut generator,
        &config.output_filename,
        config.row_count,
        config.col_count,
    )
    .expect("Failed to generate and write matrix");

    println!(
        "Generation took {} ms to complete",
        start_time.elapsed().as_millis()
    );
}
