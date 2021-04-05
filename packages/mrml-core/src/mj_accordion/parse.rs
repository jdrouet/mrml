use super::MJAccordion;
use super::MJAccordionChild;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJAccordionParser(MJAccordion);

impl Parser for MJAccordionParser {
    type Output = MJAccordion;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJAccordionChild);
    parse_comment!();
}

impl Parsable for MJAccordion {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAccordionParser::default().parse(tokenizer)?.build()
    }
}
