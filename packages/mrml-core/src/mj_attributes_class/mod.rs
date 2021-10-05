use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-class";

#[derive(Debug, Default)]
pub struct MJAttributesClass {
    name: String,
    attributes: Map<String, String>,
}

impl MJAttributesClass {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &Map<String, String> {
        &self.attributes
    }
}

#[cfg(test)]
impl MJAttributesClass {
    pub fn new(name: String) -> Self {
        Self {
            name,
            attributes: Map::default(),
        }
    }
}
