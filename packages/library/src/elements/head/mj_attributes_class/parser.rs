use super::MJAttributesClass;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use std::collections::HashMap;

#[derive(Default)]
struct MJAttributesClassParser {
    name: Option<String>,
    content: HashMap<String, String>,
}

impl MJMLParser for MJAttributesClassParser {
    type Output = MJAttributesClass;

    fn build(self) -> Result<Self::Output, Error> {
        if let Some(name) = self.name {
            Ok(MJAttributesClass {
                name,
                content: self.content,
            })
        } else {
            Err(Error::MissingAttribute("name".into()))
        }
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        for (key, value) in node.attributes.iter() {
            match key.as_str() {
                "name" => self.name = Some(value.to_string()),
                key_str => {
                    self.content.insert(key_str.to_string(), value.to_string());
                }
            };
        }
        Ok(self)
    }
}

impl MJAttributesClass {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJAttributesClassParser::default().parse(node)?.build()
    }
}
