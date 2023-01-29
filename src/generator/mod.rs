mod number_generator;

pub use number_generator::NumberGenerator;

pub trait Generator {
    fn supply_line_start(&self) -> String;
    fn supply_line_end(&self) -> String;
    fn supply_element(&mut self) -> String;
    fn supply_col_delimiter(&self) -> String;
}

// TODO: Generator enum
// TODO: Generator factory