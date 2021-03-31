use super::MJSpacer;
use crate::parse_attribute;
use crate::prelude::parse::{Error, Parser};
use xmlparser::Tokenizer;

#[derive(Debug, Default)]
struct MJSpacerParser(MJSpacer);

impl Parser for MJSpacerParser {
    type Output = MJSpacer;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
}

impl MJSpacer {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJSpacerParser::default().parse(tokenizer)?.build()
    }
}
