use std::i128;
use std::ops::Range;

use rand::distributions::Uniform;
use rand::prelude::*;

use super::constants::{EMPTY_STRING, NEW_LINE};
use super::Generator;

pub struct NumberGenerator {
    rng: StdRng,
    uniform: Uniform<i128>,
    column_delimiter: String,
}

impl NumberGenerator {
    pub fn new(range: Range<i128>, column_delimiter: &str) -> Self {
        NumberGenerator {
            rng: StdRng::from_entropy(),
            column_delimiter: column_delimiter.to_owned(),
            uniform: Uniform::from(range),
        }
    }
}

impl Generator<i128> for NumberGenerator {
    fn supply_line_start(&self) -> &str {
        EMPTY_STRING
    }

    fn supply_line_end(&self) -> &str {
        NEW_LINE
    }

    fn supply_element(&mut self) -> i128 {
        self.uniform.sample(&mut self.rng)
    }

    fn supply_col_delimiter(&self) -> &str {
        &self.column_delimiter
    }
}
