pub trait Generate {
    fn supply_line_start(&self) -> String;
    fn supply_line_end(&self) -> String;
    fn supply_element(&mut self) -> String;
    fn supply_col_delimiter(&self) -> String;
}
