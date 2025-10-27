use crate::{AsciiGenerator, Config, Generator, Generators, NumberGenerator};

pub fn generator_from_config(config: &Config) -> Result<Box<dyn Generator>, anyhow::Error> {
    let delimiter = &config.common_args.delimiter;

    let generator: Box<dyn Generator> = match &config.generator {
        Generators::Number(args) => Box::new(NumberGenerator::new(args.range(), delimiter)?),
        Generators::Ascii(args) => Box::new(AsciiGenerator::new(args.list.to_owned(), delimiter)?),
    };

    Ok(generator)
}
