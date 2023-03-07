use std::i128;
use std::ops::Range;

use rand::rngs::ThreadRng;
use rand::Rng;

use super::constants::{EMPTY_STRING, NEW_LINE};
use super::Generator;

pub struct NumberGenerator {
    rng: ThreadRng,
    column_delimiter: String,
    range: Range<i128>,
}

impl NumberGenerator {
    pub fn new(range: Range<i128>, column_delimiter: &str) -> Self {
        NumberGenerator {
            rng: rand::thread_rng(),
            column_delimiter: column_delimiter.to_owned(),
            range,
        }
    }
}

impl Generator for NumberGenerator {
    fn supply_line_start(&mut self) -> String {
        EMPTY_STRING.to_owned()
    }

    fn supply_line_end(&mut self) -> String {
        NEW_LINE.to_owned()
    }

    fn supply_element(&mut self) -> String {
        let range = self.range.to_owned();

        self.rng.gen_range(range).to_string()
    }

    fn supply_col_delimiter(&mut self) -> String {
        self.column_delimiter.to_owned()
    }
}
