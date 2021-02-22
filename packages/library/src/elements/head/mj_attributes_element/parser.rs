use super::MJAttributesElement;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use std::collections::HashMap;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Default)]
struct MJAttributesElementParser {
    name: String,
    content: HashMap<String, String>,
}

impl MJAttributesElementParser {
    pub fn new(tag: StrSpan) -> Self {
        Self {
            name: tag.to_string(),
            content: HashMap::default(),
        }
    }
}

impl MJMLParser for MJAttributesElementParser {
    type Output = MJAttributesElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAttributesElement {
            name: self.name,
            content: self.content,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.content.insert(name.to_string(), value.to_string());
        Ok(())
    }
}

impl MJAttributesElement {
    pub fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAttributesElementParser::new(tag)
            .parse(tokenizer)?
            .build()
    }
}
