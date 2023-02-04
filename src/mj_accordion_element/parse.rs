use super::{MjAccordionElement, MjAccordionElementChildren};
use crate::mj_accordion_text::{MjAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::mj_accordion_title::{MjAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::parse_attribute;
use crate::prelude::hash::Map;
use crate::prelude::parse::{Error, Parsable, Parser, ParserOptions};
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MjAccordionElementParser {
    opts: Rc<ParserOptions>,
    attributes: Map<String, String>,
    children: MjAccordionElementChildren,
}

impl MjAccordionElementParser {
    fn new(opts: Rc<ParserOptions>) -> Self {
        Self {
            opts,
            attributes: Map::default(),
            children: Default::default(),
        }
    }
}

impl Parser for MjAccordionElementParser {
    type Output = MjAccordionElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MjAccordionElement {
            attributes: self.attributes,
            children: self.children,
        })
    }

    parse_attribute!();

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        match tag.as_str() {
            MJ_ACCORDION_TEXT => {
                self.children.text =
                    Some(MjAccordionText::parse(tag, tokenizer, self.opts.clone())?)
            }
            MJ_ACCORDION_TITLE => {
                self.children.title =
                    Some(MjAccordionTitle::parse(tag, tokenizer, self.opts.clone())?)
            }
            _ => return Err(Error::UnexpectedElement(tag.start())),
        };
        Ok(())
    }
}

impl Parsable for MjAccordionElement {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjAccordionElementParser::new(opts)
            .parse(tokenizer)?
            .build()
    }
}
