use std::collections::HashMap;
use std::ops::Deref;

use super::MJAttributesClass;
use crate::elements::error::Error;
use crate::parser::Node;

impl MJAttributesClass {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        let mut content = HashMap::new();
        let mut name = String::new();
        for (key, value) in node.attributes.iter() {
            match key.deref() {
                "name" => name = value.to_string(),
                key_str => {
                    content.insert(key_str.to_string(), value.to_string());
                }
            };
        }
        if name.is_empty() {
            Err(Error::MissingAttribute("name".into()))
        } else {
            Ok(Self { content, name })
        }
    }
}
