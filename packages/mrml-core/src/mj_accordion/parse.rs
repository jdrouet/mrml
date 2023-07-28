use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjAccordion, MjAccordionChild};
use crate::comment::Comment;
use crate::mj_accordion_element::{MjAccordionElement, NAME as MJ_ACCORDION_ELEMENT};
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken, Parsable,
    ParserOptions,
};

impl<'a> ChildrenParser<'a, Vec<MjAccordionChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjAccordionChild>, Error> {
        let mut result = Vec::new();

        loop {
            match self.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjAccordionChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_ACCORDION_ELEMENT {
                        result.push(MjAccordionChild::MjAccordionElement(
                            self.parse(inner.local)?,
                        ));
                    } else {
                        return Err(Error::UnexpectedElement(inner.span.start()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::unexpected_token(other.range())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjAccordion> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordion, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;

        if ending.empty {
            return Ok(MjAccordion {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjAccordion {
            attributes,
            children,
        })
    }
}

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
