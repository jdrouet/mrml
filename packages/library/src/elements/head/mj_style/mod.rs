use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::header::Header;

#[derive(Clone, Debug)]
pub struct MJStyle {
    content: String,
    inline: bool,
}

impl MJStyle {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        let mut content = String::new();
        let mut inline = false;
        for (key, _value) in node.attributes.iter() {
            match key.as_str() {
                "inline" => {
                    inline = true;
                }
                name => return Err(Error::UnexpectedAttribute(name.into())),
            };
        }
        for child in node.children.iter() {
            match child {
                Element::Text(value) => content.push_str(value.as_str()),
                _ => return Err(Error::InvalidChild),
            };
        }
        Ok(Self { inline, content })
    }
}

impl HeadComponent for MJStyle {
    fn update_header(&self, header: &mut Header) {
        if self.inline {
            // TODO
        } else {
            header.add_style(self.content.clone());
        }
    }
}
