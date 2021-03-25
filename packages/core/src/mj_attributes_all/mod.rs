use std::collections::HashMap;

mod parse;
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
