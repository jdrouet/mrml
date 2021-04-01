use super::MJSpacer;
use crate::parse_attribute;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJSpacerParser(MJSpacer);

impl Parser for MJSpacerParser {
    type Output = MJSpacer;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
}

impl Parsable for MJSpacer {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJSpacerParser::default().parse(tokenizer)?.build()
    }
}
