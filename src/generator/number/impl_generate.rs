use super::NumberGenerator;
use crate::generator::Generate;

impl Generate for NumberGenerator {
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
