use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::generator::Generator;

pub fn write_matrix_into_file<T: Display>(
    generator: &mut impl Generator<T>,
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

pub fn write_matrix<T: Display>(
    generator: &mut impl Generator<T>,
    writable: &mut impl Write,
    row_count: u128,
    col_count: u128,
) -> Result<(), io::Error> {
    let mut writer = BufWriter::new(writable);

    for _row in 0..row_count {
        write!(writer, "{}", generator.supply_line_start())?;

        for _col in 0..(col_count - 1) {
            write!(writer, "{}", generator.supply_element())?;
            write!(writer, "{}", generator.supply_col_delimiter())?;
        }

        // write the last column separately to write line_end instead of column_delimiter
        write!(writer, "{}", generator.supply_element())?;
        write!(writer, "{}", generator.supply_line_end())?;
    }

    Ok(())
}
