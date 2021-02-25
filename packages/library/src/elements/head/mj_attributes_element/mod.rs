use std::collections::HashMap;

mod parser;
mod renderer;

#[derive(Debug, Default)]
pub struct MJAttributesElement {
    name: String,
    content: HashMap<String, String>,
}
