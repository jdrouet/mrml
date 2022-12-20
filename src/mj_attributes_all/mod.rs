use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-all";

#[derive(Debug, Default)]
pub struct MJAttributesAll {
    attributes: Map<String, String>,
}

impl MJAttributesAll {
    pub fn attributes(&self) -> &Map<String, String> {
        &self.attributes
    }
}
