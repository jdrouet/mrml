use super::MJPreview;
use crate::elements::error::Error;
use crate::parser::{Element, Node};

impl MJPreview {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        match node.children.first() {
            Some(element) => match element {
                Element::Text(value) => Ok(Self {
                    content: value.as_str().into(),
                }),
                _ => Err(Error::InvalidChild),
            },
            None => Ok(Self::default()),
        }
    }
}
