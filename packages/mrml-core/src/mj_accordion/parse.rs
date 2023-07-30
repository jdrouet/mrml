use xmlparser::StrSpan;

use super::{MjAccordion, MjAccordionChild};
use crate::comment::Comment;
use crate::mj_accordion_element::NAME as MJ_ACCORDION_ELEMENT;
use crate::prelude::parser::{ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken};

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
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjAccordion> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordion, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjAccordion {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordion;
    use crate::prelude::parser::MrmlParser;

    macro_rules! should_success {
        ($title:ident, $template:expr) => {
            #[test]
            fn $title() {
                let raw = $template;
                let _: MjAccordion = MrmlParser::new(raw, Default::default())
                    .parse_root()
                    .unwrap();
            }
        };
    }

    macro_rules! should_fail {
        ($title:ident, $template:expr, $error:expr) => {
            #[test]
            #[should_panic(expected = $error)]
            fn $title() {
                let raw = $template;
                let _: MjAccordion = MrmlParser::new(raw, Default::default())
                    .parse_root()
                    .unwrap();
            }
        };
    }

    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-accordion.mjml");
        let result: crate::mjml::Mjml =
            crate::prelude::parser::MrmlParser::new(template, Default::default())
                .parse_root()
                .unwrap();
        assert!(!format!("{result:?}").is_empty());
    }

    should_success!(
        should_keep_comments,
        "<mj-accordion><!-- comment --></mj-accordion>"
    );

    should_success!(should_work_empty, "<mj-accordion />");

    should_fail!(
        should_error_with_text,
        "<mj-accordion>Hello</mj-accordion>",
        "UnexpectedToken(Span { start: 14, end: 19 })"
    );

    should_fail!(
        should_error_with_unknown_element,
        "<mj-accordion><span /></mj-accordion>",
        "UnexpectedElement(Span { start: 14, end: 19 })"
    );
}
