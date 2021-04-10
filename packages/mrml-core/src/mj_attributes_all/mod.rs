use std::collections::HashMap;

#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-all";

#[derive(Debug, Default)]
pub struct MJAttributesAll {
    attributes: HashMap<String, String>,
}

impl MJAttributesAll {
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }
}
