use std::collections::HashMap;

use super::MJAttributesAll;
use crate::elements::error::Error;
use crate::parser::Node;

impl MJAttributesAll {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        Ok(Self {
            content: node
                .attributes
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<HashMap<String, String>>(),
        })
    }
}
