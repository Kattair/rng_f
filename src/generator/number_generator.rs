use std::i128;
use std::ops::Range;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::constants::{EMPTY_STRING, NEW_LINE};
use super::Generator;

pub struct NumberGenerator {
    buffer: String,
    rng: SmallRng,
    range: Range<i128>,
    column_delimiter: String,
}

impl NumberGenerator {
    pub fn new(range: Range<i128>, column_delimiter: &str) -> Self {
        NumberGenerator {
            buffer: String::with_capacity(32),
            rng: rand::rngs::SmallRng::from_entropy(),
            column_delimiter: column_delimiter.to_owned(),
            range,
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
        self.buffer.clear();
        
        let element = self.rng.gen_range(self.range.clone());
        self.buffer.push_str(&element.to_string());
        
        &self.buffer
    }

    fn supply_col_delimiter(&self) -> &str {
        &self.column_delimiter
    }
}
