use std::str::FromStr;

use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
};

use crate::{
    Generator,
    generator::constants::{EMPTY_STRING, NEW_LINE},
};

pub struct AsciiGenerator {
    rng: StdRng,
    uniform: Uniform<u8>,
    column_delimiter: String,
    chars: Vec<String>,
}

impl AsciiGenerator {
    pub fn new(chars: String, delimiter: &str) -> Result<Self, anyhow::Error> {
        Ok(AsciiGenerator {
            rng: StdRng::from_os_rng(),
            column_delimiter: delimiter.to_owned(),
            uniform: Uniform::try_from(0..chars.len() as u8)?,
            chars: chars
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(String::from)
                .collect(),
        })
    }
}

impl Generator for AsciiGenerator {
    fn supply_line_start(&self) -> &str {
        EMPTY_STRING
    }

    fn supply_line_end(&self) -> &str {
        NEW_LINE
    }

    fn supply_element(&mut self) -> &str {
        let number = self.uniform.sample(&mut self.rng);

        &self.chars[number as usize]
    }

    fn supply_col_delimiter(&self) -> &str {
        &self.column_delimiter
    }
}
