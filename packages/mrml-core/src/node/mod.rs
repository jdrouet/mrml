use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[derive(Debug, Default)]
pub struct Node<T> {
    tag: String,
    attributes: Map<String, String>,
    children: Vec<T>,
}

impl<T> Node<T> {
    pub fn new(tag: String) -> Self {
        Self::from(tag)
    }
}

impl<T> From<String> for Node<T> {
    fn from(tag: String) -> Self {
        Self {
            tag,
            attributes: Map::new(),
            children: Vec::new(),
        }
    }
}

impl<T> From<&str> for Node<T> {
    fn from(tag: &str) -> Self {
        Self::from(tag.to_string())
    }
}
