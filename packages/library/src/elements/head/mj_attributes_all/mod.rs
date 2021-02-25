use std::collections::HashMap;

mod parser;
mod renderer;

pub const NAME: &str = "mj-all";

#[derive(Debug, Default)]
pub struct MJAttributesAll {
    content: HashMap<String, String>,
}
