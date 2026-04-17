use std::ops::Range;

use rand::distr::Uniform;
use rand::prelude::*;
use rand::rngs::SysRng;

use super::Generator;
use super::constants::{EMPTY_STRING, NEW_LINE};

pub struct NumberGenerator {
    rng: StdRng,
    uniform: Uniform<i64>,
    column_delimiter: String,
    buffer: itoa::Buffer,
}

impl NumberGenerator {
    pub fn new(range: Range<i64>, delimiter: &str) -> Result<Self, anyhow::Error> {
        Ok(NumberGenerator {
            rng: StdRng::try_from_rng(&mut SysRng).unwrap(),
            column_delimiter: delimiter.to_owned(),
            uniform: Uniform::try_from(range)?,
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
