use std::collections::HashMap;

mod parser;
mod renderer;

pub const NAME: &str = "mj-class";

#[derive(Clone, Debug, Default)]
pub struct MJAttributesClass {
    name: String,
    content: HashMap<String, String>,
}
