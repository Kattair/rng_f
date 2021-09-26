use crate::generator::Generate;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Writer {
    generator: Box<dyn Generate>,
}

impl Writer {
    pub fn new(generator: Box<dyn Generate>) -> Writer {
        Writer { generator }
    }

    pub fn write_matrix(
        &mut self,
        filename: &str,
        row_count: u128,
        col_count: u128,
    ) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::create_file_writer(filename)?;

        for _row in 0..row_count {
            write!(writer, "{}", self.generator.supply_line_start())?;

            for _col in 0..col_count {
                write!(writer, "{}", self.generator.supply_element())?;
                write!(writer, "{}", self.generator.supply_col_delimiter())?;
            }

            write!(writer, "{}", self.generator.supply_line_end())?;
        }

        writer.flush()?;

        Ok(())
    }

    fn create_file_writer(filename: &str) -> Result<BufWriter<File>, Box<dyn Error>> {
        let path = Path::new(filename);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        Ok(BufWriter::new(file))
    }
}
