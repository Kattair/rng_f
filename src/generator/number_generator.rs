use std::i128;
use std::ops::Range;

use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use super::constants::{EMPTY_STRING, NEW_LINE};
use super::Generator;

pub struct NumberGenerator {
    buffer: String,
    rng: SmallRng,
    uniform: Uniform<i128>,
    column_delimiter: String,
}

impl NumberGenerator {
    pub fn new(range: Range<i128>, column_delimiter: &str) -> Self {
        NumberGenerator {
            buffer: String::new(),
            rng: rand::rngs::SmallRng::from_entropy(),
            column_delimiter: column_delimiter.to_owned(),
            uniform: Uniform::from(range),
        }
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
        let element = self.uniform.sample(&mut self.rng);
        self.buffer = element.to_string();

        &self.buffer
    }

    fn supply_col_delimiter(&self) -> &str {
        &self.column_delimiter
    }
}
