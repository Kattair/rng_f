use rng_f::{config::Config, generator::NumberGenerator, writer};

fn main() {
    let config = Config::new().expect("Failed to parse command line arguments");
    let range = match config.range {
        Some(range) => range[0]..range[1],
        None => i128::MIN..i128::MAX,
    };
    let mut generator = NumberGenerator::new(range, &config.delimiter);

    writer::write_matrix(
        &mut generator,
        &config.output_filename,
        config.row_count,
        config.col_count,
    )
    .expect("Failed to generate and write matrix");
}
