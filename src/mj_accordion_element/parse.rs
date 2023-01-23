use super::MjAccordionElement;
use crate::mj_accordion_text::{MjAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::mj_accordion_title::{MjAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::parse_attribute;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MjAccordionElementParser(MjAccordionElement);

impl Parser for MjAccordionElementParser {
    type Output = MjAccordionElement;

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
            MJ_ACCORDION_TEXT => {
                self.0.children.text = Some(MjAccordionText::parse(tag, tokenizer)?)
            }
            MJ_ACCORDION_TITLE => {
                self.0.children.title = Some(MjAccordionTitle::parse(tag, tokenizer)?)
            }
            _ => return Err(Error::UnexpectedElement(tag.start())),
        };
        Ok(())
    }
}

impl Parsable for MjAccordionElement {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MjAccordionElementParser::default()
            .parse(tokenizer)?
            .build()
    }
}
