use std::collections::HashMap;

mod parser;
mod renderer;

#[derive(Clone, Debug, Default)]
pub struct MJAttributesElement {
    name: String,
    content: HashMap<String, String>,
}
