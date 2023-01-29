use rand::rngs::ThreadRng;
use rand::Rng;

use crate::Config;
use super::Generator;

static EMPTY_STRING: &str = "";
static NEW_LINE: &str = "\n";

pub struct NumberGenerator {
    pub config: Config,
    pub rng: ThreadRng,

    pub line_start_supplier: fn(&Config) -> String,
    pub line_end_supplier: fn(&Config) -> String,
    pub element_supplier: fn(&Config, &mut ThreadRng) -> String,
    pub col_delimiter_supplier: fn(&Config) -> String,
}

impl NumberGenerator {
    pub fn from_config(config: &Config) -> Box<dyn Generator> {
        Box::new(NumberGenerator {
            rng: rand::thread_rng(),
            line_start_supplier: NumberGenerator::create_line_start_supplier(&config),
            line_end_supplier: NumberGenerator::create_line_end_supplier(&config),
            element_supplier: NumberGenerator::create_element_supplier(&config),
            col_delimiter_supplier: NumberGenerator::create_delimiter_supplier(&config),
            config: config.clone(),
        })
    }

    fn create_line_start_supplier(_config: &Config) -> fn(&Config) -> String {
        |_config: &Config| EMPTY_STRING.to_string()
    }

    fn create_line_end_supplier(_config: &Config) -> fn(&Config) -> String {
        |_config: &Config| NEW_LINE.to_string()
    }

    fn create_element_supplier(config: &Config) -> fn(&Config, &mut ThreadRng) -> String {
        if let Some(_) = &config.range {
            return |config: &Config, rng: &mut ThreadRng| {
                let range = &config.range.as_ref().unwrap();

                rng.gen_range(range[0]..range[1]).to_string()
            };
        }

        |_config: &Config, rng: &mut ThreadRng| rng.gen::<i128>().to_string()
    }

    fn create_delimiter_supplier(_config: &Config) -> fn(&Config) -> String {
        |config: &Config| String::from(&config.delimiter)
    }
}

impl Generator for NumberGenerator {
    fn supply_line_start(&self) -> String {
        (self.line_start_supplier)(&self.config)
    }

    fn supply_line_end(&self) -> String {
        (self.line_end_supplier)(&self.config)
    }

    fn supply_element(&mut self) -> String {
        (self.element_supplier)(&self.config, &mut self.rng)
    }

    fn supply_col_delimiter(&self) -> String {
        (self.col_delimiter_supplier)(&self.config)
    }
}