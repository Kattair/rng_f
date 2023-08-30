mod constants;
mod number_generator;

use std::fmt::Display;

pub use number_generator::NumberGenerator;

pub trait Generator<T: Display> {
    fn supply_line_start(&self) -> &str;
    fn supply_line_end(&self) -> &str;
    fn supply_element(&mut self) -> T;
    fn supply_col_delimiter(&self) -> &str;
}
