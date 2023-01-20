use super::MJAccordionChild;
use crate::mj_accordion_element::{MJAccordionElement, NAME as MJ_ACCORDION_ELEMENT};
use crate::prelude::parse::{Error, Parsable};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MJAccordionChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION_ELEMENT => Ok(MJAccordionElement::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
