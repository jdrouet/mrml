use super::MJAccordion;
use super::MJAccordionChild;
use crate::mj_accordion_element::{MJAccordionElement, NAME as MJ_ACCORDION_ELEMENT};
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

impl Parsable for MJAccordionChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION_ELEMENT => Ok(MJAccordionElement::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
