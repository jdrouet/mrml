use super::MJTitle;
use crate::elements::error::Error;
use crate::parser::{Element, Node};

impl MJTitle {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        let mut content = String::new();
        for child in node.children.iter() {
            match child {
                Element::Text(value) => content.push_str(value.as_str()),
                _ => return Err(Error::InvalidChild),
            }
        }
        Ok(Self { content })
    }
}
