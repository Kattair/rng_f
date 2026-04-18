use std::ops::Range;

use rand::distr::Uniform;
use rand::prelude::*;
use rand::rngs::SysRng;

use super::constants::{EMPTY_STRING, NEW_LINE};
use super::{Generator, SizeHint};

pub struct NumberGenerator {
    rng: StdRng,
    uniform: Uniform<i64>,
    column_delimiter: String,
    buffer: itoa::Buffer,
    max_element_bytes: usize,
}

impl NumberGenerator {
    pub fn new(range: Range<i64>, delimiter: &str) -> Result<Self, anyhow::Error> {
        let max_element_bytes = NumberGenerator::calculate_max_element_size(&range);

        Ok(NumberGenerator {
            rng: StdRng::try_from_rng(&mut SysRng)
                .map_err(|e| anyhow::anyhow!("failed to seed RNG: {e}"))?,
            column_delimiter: delimiter.to_owned(),
            uniform: Uniform::try_from(range)?,
            buffer: itoa::Buffer::new(),
            max_element_bytes,
        })
    }

    fn calculate_max_element_size(range: &Range<i64>) -> usize {
        let mut buf = itoa::Buffer::new();

        let start_len = buf.format(range.start).len();
        let end_len = buf.format(range.end - 1).len();

        start_len.max(end_len)
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

    fn size_hint(&self) -> Option<SizeHint> {
        Some(SizeHint {
            line_start_bytes: self.supply_line_start().len(),
            line_end_bytes: self.supply_line_end().len(),
            max_element_bytes: self.max_element_bytes,
            col_delimiter_bytes: self.column_delimiter.len(),
        })
    }
}
