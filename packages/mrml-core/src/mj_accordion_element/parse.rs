use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjAccordionElement, MjAccordionElementChildren};
use crate::mj_accordion_text::{MjAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::mj_accordion_title::{MjAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::parse_attribute;
use crate::prelude::hash::Map;
use crate::prelude::parser::{Error, Parsable, Parser, ParserOptions};

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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use xmlparser::Tokenizer;

    use crate::mj_accordion_element::MjAccordionElementChild;
    use crate::prelude::parser::{is_element_start, next_token, Error, Parsable, ParserOptions};

    #[test]
    fn parse_title_child() {
        let opts = Rc::new(ParserOptions::default());
        let mut tokenizer = Tokenizer::from("<mj-accordion-title>Hello</mj-accordion-title>");
        let token = next_token(&mut tokenizer).unwrap();
        let tag = is_element_start(&token).unwrap();
        let elt = MjAccordionElementChild::parse(*tag, &mut tokenizer, opts).unwrap();
        assert!(elt.as_mj_accordion_title().is_some())
    }

    #[test]
    fn parse_title_child_errored() {
        let opts = Rc::new(ParserOptions::default());
        let mut tokenizer =
            Tokenizer::from("<mj-accordion-title><span>Hello</span></mj-accordion-title>");
        let token = next_token(&mut tokenizer).unwrap();
        let tag = is_element_start(&token).unwrap();
        let err = MjAccordionElementChild::parse(*tag, &mut tokenizer, opts).unwrap_err();
        assert!(matches!(err, Error::UnexpectedElement(21)));
    }

    #[test]
    fn parse_text_child() {
        let opts = Rc::new(ParserOptions::default());
        let mut tokenizer = Tokenizer::from("<mj-accordion-text>Hello</mj-accordion-text>");
        let token = next_token(&mut tokenizer).unwrap();
        let tag = is_element_start(&token).unwrap();
        let elt = MjAccordionElementChild::parse(*tag, &mut tokenizer, opts).unwrap();
        assert!(elt.as_mj_accordion_text().is_some());
    }

    #[test]
    fn parse_text_child_errored() {
        let opts = Rc::new(ParserOptions::default());
        let mut tokenizer = Tokenizer::from("<mj-accordion-text>");
        let token = next_token(&mut tokenizer).unwrap();
        let tag = is_element_start(&token).unwrap();
        let err = MjAccordionElementChild::parse(*tag, &mut tokenizer, opts).unwrap_err();
        assert!(matches!(err, Error::InvalidFormat(_, _)));
    }

    #[test]
    fn parse_unknown_child() {
        let opts = Rc::new(ParserOptions::default());
        let mut tokenizer = Tokenizer::from("<mj-pouwet>Hello</mj-pouwet>");
        let token = next_token(&mut tokenizer).unwrap();
        let tag = is_element_start(&token).unwrap();
        assert!(MjAccordionElementChild::parse(*tag, &mut tokenizer, opts).is_err());
    }
}
