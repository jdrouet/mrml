use std::collections::HashMap;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
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

impl MJAttributesElement {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }
}
