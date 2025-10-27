use crate::{Config, Generator, Generators, NumberGenerator};

pub fn generator_from_config(config: &Config) -> Result<impl Generator, anyhow::Error> {
    match &config.generator {
        Generators::Number(args) => {
            NumberGenerator::new(args.range(), &config.common_args.delimiter)
        }
        _ => todo!(),
    }
}
