use std::collections::HashMap;

mod parse;
mod print;

#[derive(Debug, Default)]
pub struct MJAttributesElement {
    name: String,
    attributes: HashMap<String, String>,
}

impl MJAttributesElement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            attributes: HashMap::new(),
        }
    }
}
