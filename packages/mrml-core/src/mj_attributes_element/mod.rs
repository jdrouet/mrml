use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

#[derive(Debug, Default)]
pub struct MjAttributesElement {
    pub name: String,
    pub attributes: Map<String, String>,
}

impl MjAttributesElement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            attributes: Map::new(),
        }
    }
}

impl MjAttributesElement {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn attributes(&self) -> &Map<String, String> {
        &self.attributes
    }
}
