use std::collections::HashMap;

use super::MJAttributesElement;
use crate::elements::error::Error;
use crate::parser::Node;

impl MJAttributesElement {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        Ok(Self {
            name: node.name.to_string(),
            content: node
                .attributes
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<HashMap<String, String>>(),
        })
    }
}
