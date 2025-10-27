mod ascii_generator;
mod constants;
mod number_generator;

use crate::{Config, Generators};

pub use ascii_generator::AsciiGenerator;
pub use number_generator::NumberGenerator;

#[enum_delegate::register]
pub trait Generator {
    fn supply_line_start(&self) -> &str;
    fn supply_line_end(&self) -> &str;
    fn supply_element(&mut self) -> &str;
    fn supply_col_delimiter(&self) -> &str;
}

#[enum_delegate::implement(Generator)]
pub enum GeneratorDispatch {
    NumberGenerator(NumberGenerator),
    AsciiGenerator(AsciiGenerator),
}

pub fn generator_from_config(config: &Config) -> Result<impl Generator, anyhow::Error> {
    let delimiter = &config.common_args.delimiter;

    let generator: GeneratorDispatch = match &config.generator {
        Generators::Number(args) => {
            GeneratorDispatch::from(NumberGenerator::new(args.range(), delimiter)?)
        }
        Generators::Ascii(args) => {
            GeneratorDispatch::from(AsciiGenerator::new(args.list.to_owned(), delimiter)?)
        }
    };

    Ok(generator)
}
