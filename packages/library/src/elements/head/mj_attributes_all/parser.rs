use std::collections::HashMap;

use super::MJAttributesAll;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};

#[derive(Default)]
struct MJAttributesAllParser {
    content: HashMap<String, String>,
}

impl MJMLParser for MJAttributesAllParser {
    type Output = MJAttributesAll;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAttributesAll {
            content: self.content,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.content = node
            .attributes
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();
        Ok(self)
    }
}

impl MJAttributesAll {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJAttributesAllParser::default().parse(node)?.build()
    }
}
