use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag_field = "tag"))]
pub struct Node<T> {
    pub tag: String,
    pub attributes: Map<String, String>,
    pub children: Vec<T>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            tag: Default::default(),
            attributes: Default::default(),
            children: Default::default(),
        }
    }
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
