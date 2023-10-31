use std::i64;
use std::ops::Range;

use rand::distributions::Uniform;
use rand::prelude::*;
use thiserror::Error;

use super::constants::{EMPTY_STRING, NEW_LINE};
use super::Generator;

#[derive(Debug, Error)]
pub enum NumberGeneratorError {
    #[error("the provided range <{0:?}) contains no elements")]
    EmptyRangeError(Range<i64>),
}

pub struct NumberGenerator {
    rng: StdRng,
    uniform: Uniform<i64>,
    column_delimiter: String,
    buffer: itoa::Buffer,
}

impl NumberGenerator {
    pub fn new(range: Range<i64>, delimiter: &str) -> Result<Self, NumberGeneratorError> {
        if range.is_empty() {
            return Err(NumberGeneratorError::EmptyRangeError(range));
        }

        Ok(NumberGenerator {
            rng: StdRng::from_entropy(),
            column_delimiter: delimiter.to_owned(),
            uniform: Uniform::from(range),
            buffer: itoa::Buffer::new(),
        })
    }
}

impl Generator for NumberGenerator {
    fn supply_line_start(&self) -> &str {
        EMPTY_STRING
    }

    fn supply_line_end(&self) -> &str {
        NEW_LINE
    }

    fn supply_element(&mut self) -> &str {
        let number = self.uniform.sample(&mut self.rng);

        self.buffer.format(number)
    }

    fn supply_col_delimiter(&self) -> &str {
        &self.column_delimiter
    }
}
