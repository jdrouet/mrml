use crate::prelude::{hash::Map, AttributeMap};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

#[derive(Clone, Debug, Default)]
pub struct MjAttributesElement {
    pub name: String,
    pub attributes: AttributeMap,
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

    pub fn attributes(&self) -> &AttributeMap {
        &self.attributes
    }
}
