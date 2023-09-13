use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::generator::Generator;

pub fn write_matrix_into_file(
    generator: &mut impl Generator,
    filename: &str,
    row_count: u128,
    col_count: u128,
) -> Result<(), io::Error> {
    let path = Path::new(filename);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    write_matrix(generator, &mut file, row_count, col_count)
}

pub fn write_matrix(
    generator: &mut impl Generator,
    writable: &mut impl Write,
    row_count: u128,
    col_count: u128,
) -> Result<(), io::Error> {
    let mut writer = BufWriter::new(writable);

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
