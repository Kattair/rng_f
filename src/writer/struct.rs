use crate::generator::Generate;

pub struct Writer {
    pub generator: Box<dyn Generate>,
}
