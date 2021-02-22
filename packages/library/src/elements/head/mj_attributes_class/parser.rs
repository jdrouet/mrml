use super::MJAttributesClass;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use std::collections::HashMap;
use xmlparser::{StrSpan, Tokenizer};

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

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "name" {
            self.name = Some(value.to_string());
        } else {
            self.content.insert(name.to_string(), value.to_string());
        }
        Ok(())
    }
}

impl MJAttributesClass {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAttributesClassParser::default().parse(tokenizer)?.build()
    }
}
