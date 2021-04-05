use super::MJImage;
use crate::parse_attribute;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJImageParser(MJImage);

impl Parser for MJImageParser {
    type Output = MJImage;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
}

impl Parsable for MJImage {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJImageParser::default().parse(tokenizer)?.build()
    }
}
