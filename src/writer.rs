use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use anyhow::Context;

use crate::generator::{Generator, SizeHint};

pub fn write_matrix_into_file(
    generator: &mut impl Generator,
    filename: &str,
    row_count: u128,
    col_count: u128,
) -> Result<(), anyhow::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(filename))
        .with_context(|| format!("failed to open output file '{filename}'"))?;

    write_matrix(generator, &mut file, row_count, col_count)
}

pub fn write_matrix(
    generator: &mut impl Generator,
    writable: &mut impl Write,
    row_count: u128,
    col_count: u128,
) -> Result<(), anyhow::Error> {
    let mut writer =
        BufWriter::with_capacity(pick_buffer_size(generator.size_hint(), col_count), writable);

    for _row in 0..row_count {
        writer.write_all(generator.supply_line_start().as_bytes())?;

        for _col in 0..(col_count - 1) {
            writer.write_all(generator.supply_element().as_bytes())?;
            writer.write_all(generator.supply_col_delimiter().as_bytes())?;
        }

        // write the last column separately to write line_end instead of column_delimiter
        writer.write_all(generator.supply_element().as_bytes())?;
        writer.write_all(generator.supply_line_end().as_bytes())?;
    }

    Ok(())
}

fn pick_buffer_size(hint: Option<SizeHint>, col_count: u128) -> usize {
    let default_capacity = BufWriter::new(io::sink()).capacity();

    let Some(hint) = hint else {
        return default_capacity;
    };

    let col_size = hint.max_element_bytes + hint.col_delimiter_bytes;
    let est_row_bytes =
        (col_count as usize) * col_size + hint.line_start_bytes + hint.line_end_bytes;

    // Floor at BufWriter's own default so tiny rows still batch reasonably.
    est_row_bytes.max(default_capacity)
}
