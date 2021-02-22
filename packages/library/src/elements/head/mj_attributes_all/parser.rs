use super::MJAttributesAll;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use std::collections::HashMap;
use xmlparser::{StrSpan, Tokenizer};

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

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.content.insert(name.to_string(), value.to_string());
        Ok(())
    }
}

impl MJAttributesAll {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAttributesAllParser::default().parse(tokenizer)?.build()
    }
}
