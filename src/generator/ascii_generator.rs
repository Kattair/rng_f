use crate::Generator;

pub struct AsciiGenerator {}

impl Generator for AsciiGenerator {
    fn supply_line_start(&self) -> &str {
        todo!()
    }

    fn supply_line_end(&self) -> &str {
        todo!()
    }

    fn supply_element(&mut self) -> &str {
        todo!()
    }

    fn supply_col_delimiter(&self) -> &str {
        todo!()
    }
}
