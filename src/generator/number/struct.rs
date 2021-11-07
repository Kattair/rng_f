use rand::rngs::ThreadRng;

use crate::Config;

pub struct NumberGenerator {
    pub config: Config,
    pub rng: ThreadRng,

    pub line_start_supplier: fn(&Config) -> String,
    pub line_end_supplier: fn(&Config) -> String,
    pub element_supplier: fn(&Config, &mut ThreadRng) -> String,
    pub col_delimiter_supplier: fn(&Config) -> String,
}
