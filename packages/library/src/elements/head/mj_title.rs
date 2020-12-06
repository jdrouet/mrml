use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::header::Header;

#[derive(Clone, Debug)]
pub struct MJTitle {
    content: String,
}

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

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl HeadComponent for MJTitle {
    fn update_header(&self, header: &mut Header) {
        header.set_title(self.get_content());
    }
}
