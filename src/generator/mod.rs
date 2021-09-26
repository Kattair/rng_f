use crate::config::Config;
use rand::rngs::ThreadRng;
use rand::Rng;

static EMPTY_STRING: &str = "";
static NEW_LINE: &str = "\n";

pub trait Generate {
    fn supply_line_start(&self) -> String;
    fn supply_line_end(&self) -> String;
    fn supply_element(&mut self) -> String;
    fn supply_col_delimiter(&self) -> String;
}

pub struct Generator {
    config: Config,
    rng: ThreadRng,

    line_start_supplier: fn(&Config) -> String,
    line_end_supplier: fn(&Config) -> String,
    element_supplier: fn(&Config, &mut ThreadRng) -> String,
    col_delimiter_supplier: fn(&Config) -> String,
}

impl Generator {
    pub fn from_config(config: Config) -> Box<Self> {
        Box::new(
            Generator {
                rng: rand::thread_rng(),
                line_start_supplier: Generator::create_line_start_supplier(&config),
                line_end_supplier: Generator::create_line_end_supplier(&config),
                element_supplier: Generator::create_element_supplier(&config),
                col_delimiter_supplier: Generator::create_delimiter_supplier(&config),
                config,
            }
        )
    }

    fn create_line_start_supplier(_config: &Config) -> fn(&Config) -> String {
        |_config: &Config| {EMPTY_STRING.to_string()}
    }

    fn create_line_end_supplier(_config: &Config) -> fn(&Config) -> String {
        |_config: &Config| {NEW_LINE.to_string()}
    }

    fn create_element_supplier(config: &Config) -> fn(&Config, &mut ThreadRng) -> String {
        if config.custom_range {
            return |config: &Config, rng: &mut ThreadRng| {
                rng.gen_range(config.range_from..config.range_to).to_string()
            };
        }

        |_config: &Config, rng: &mut ThreadRng| {
            rng.gen::<i128>().to_string()
        }
    }

    fn create_delimiter_supplier(_config: &Config) -> fn(&Config) -> String {
        |config: &Config| {String::from(&config.col_delimiter)}
    }
}

impl Generate for Generator {
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