use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::MjAccordionChild;
use crate::mj_accordion_element::{MjAccordionElement, NAME as MJ_ACCORDION_ELEMENT};
use crate::prelude::parser::{Error, Parsable, ParserOptions};

impl Parsable for MjAccordionChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION_ELEMENT => Ok(MjAccordionElement::parse(tag, tokenizer, opts)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-accordion.mjml");
        let result = crate::mjml::Mjml::parse(template).unwrap();
        assert!(!format!("{result:?}").is_empty());
    }
}
