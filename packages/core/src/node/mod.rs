use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

mod parse;
mod print;
mod render;

#[derive(Debug, Default)]
pub struct Node {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}

impl Node {
    pub fn new(tag: String) -> Self {
        Self::from(tag)
    }
}

impl From<String> for Node {
    fn from(tag: String) -> Self {
        Self {
            tag,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }
}

impl From<&str> for Node {
    fn from(tag: &str) -> Self {
        Self::from(tag.to_string())
    }
}
