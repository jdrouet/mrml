use super::MJDivider;
use crate::parse_attribute;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJDividerParser(MJDivider);

impl Parser for MJDividerParser {
    type Output = MJDivider;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
}

impl Parsable for MJDivider {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJDividerParser::default().parse(tokenizer)?.build()
    }
}
