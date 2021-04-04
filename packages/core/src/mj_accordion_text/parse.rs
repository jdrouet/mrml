use super::MJAccordionText;
use crate::mj_raw::MJRawChild;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJAccordionTextParser(MJAccordionText);

impl Parser for MJAccordionTextParser {
    type Output = MJAccordionText;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJRawChild);
    parse_comment!();
    parse_text!();
}

impl Parsable for MJAccordionText {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAccordionTextParser::default().parse(tokenizer)?.build()
    }
}
