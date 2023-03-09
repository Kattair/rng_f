use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::generator::Generator;

pub fn write_matrix(
    generator: &mut impl Generator,
    filename: &str,
    row_count: u128,
    col_count: u128,
) -> Result<(), io::Error> {
    let mut writer = create_file_writer(filename)?;
    
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

    writer.flush()?;

    Ok(())
}

fn create_file_writer(filename: &str) -> Result<BufWriter<File>, io::Error> {
    let path = Path::new(filename);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    Ok(BufWriter::new(file))
}
