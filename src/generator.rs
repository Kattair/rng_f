mod constants;
mod number_generator;

pub use number_generator::NumberGenerator;

pub trait Generator {
    fn supply_line_start(&mut self) -> String;
    fn supply_line_end(&mut self) -> String;
    fn supply_element(&mut self) -> String;
    fn supply_col_delimiter(&mut self) -> String;
}
