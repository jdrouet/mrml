use super::MJAccordionTitle;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJAccordionTitleParser(MJAccordionTitle);

impl Parser for MJAccordionTitleParser {
    type Output = MJAccordionTitle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_text!();
}

impl Parsable for MJAccordionTitle {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAccordionTitleParser::default().parse(tokenizer)?.build()
    }
}
