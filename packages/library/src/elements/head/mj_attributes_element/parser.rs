use std::collections::HashMap;

use super::MJAttributesElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};

#[derive(Default)]
struct MJAttributesElementParser {
    name: String,
    content: HashMap<String, String>,
}

impl MJMLParser for MJAttributesElementParser {
    type Output = MJAttributesElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAttributesElement {
            name: self.name,
            content: self.content,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.name = node.name.to_string();
        self.content = node
            .attributes
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();
        Ok(self)
    }
}

impl MJAttributesElement {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJAttributesElementParser::default().parse(node)?.build()
    }
}
