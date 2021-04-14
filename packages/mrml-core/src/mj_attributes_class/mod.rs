use std::collections::HashMap;

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
    attributes: HashMap<String, String>,
}

impl MJAttributesClass {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }
}
