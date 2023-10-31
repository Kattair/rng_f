use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use thiserror::Error;

use crate::generator::Generator;

#[derive(Debug, Error)]
pub enum WriterError {
    #[error("failed to open output file '{0}'")]
    FailedToOpenFileError(io::Error),
    #[error("failed writing to output - {0}")]
    FailedToWriteError(io::Error),
}

pub fn write_matrix_into_file(
    generator: &mut impl Generator,
    filename: &str,
    row_count: u128,
    col_count: u128,
) -> Result<(), WriterError> {
    let path = Path::new(filename);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(WriterError::FailedToOpenFileError)?;

    write_matrix(generator, &mut file, row_count, col_count)
}

pub fn write_matrix(
    generator: &mut impl Generator,
    writable: &mut impl Write,
    row_count: u128,
    col_count: u128,
) -> Result<(), WriterError> {
    do_write_matrix(generator, writable, row_count, col_count)
        .map_err(WriterError::FailedToWriteError)
}

pub fn do_write_matrix(
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
