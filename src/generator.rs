mod constants;
mod number_generator;

pub use number_generator::NumberGenerator;

pub trait Generator {
    fn supply_line_start(&self) -> &str;
    fn supply_line_end(&self) -> &str;
    fn supply_element(&mut self) -> &str;
    fn supply_col_delimiter(&self) -> &str;
}
