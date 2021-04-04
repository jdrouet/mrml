use super::MJAccordionElement;
use crate::mj_accordion_text::{MJAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::mj_accordion_title::{MJAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::parse_attribute;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJAccordionElementParser(MJAccordionElement);

impl Parser for MJAccordionElementParser {
    type Output = MJAccordionElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        match tag.as_str() {
            MJ_ACCORDION_TEXT => self.0.text = Some(MJAccordionText::parse(tag, tokenizer)?),
            MJ_ACCORDION_TITLE => self.0.title = Some(MJAccordionTitle::parse(tag, tokenizer)?),
            _ => return Err(Error::UnexpectedElement(tag.start())),
        };
        Ok(())
    }
}

impl Parsable for MJAccordionElement {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAccordionElementParser::default()
            .parse(tokenizer)?
            .build()
    }
}
