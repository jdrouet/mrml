use xmlparser::StrSpan;

use super::{MjAccordion, MjAccordionChild};
use crate::comment::Comment;
use crate::mj_accordion_element::NAME as MJ_ACCORDION_ELEMENT;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<Vec<MjAccordionChild>> for MrmlParser {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjAccordionChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjAccordionChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_ACCORDION_ELEMENT {
                        result.push(MjAccordionChild::MjAccordionElement(
                            self.parse(cursor, inner.local)?,
                        ));
                    } else {
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl ParseElement<MjAccordion> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordion, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjAccordion {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordion;
    use crate::mjml::Mjml;
    use crate::prelude::parser::{MrmlCursor, MrmlParser};

    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-accordion.mjml");
        let mut cursor = MrmlCursor::new(template);
        let result: Mjml = MrmlParser::default().parse_root(&mut cursor).unwrap();
        assert!(!format!("{result:?}").is_empty());
    }

    crate::should_sync_parse!(
        should_keep_comments,
        MjAccordion,
        "<mj-accordion><!-- comment --></mj-accordion>"
    );

    crate::should_sync_parse!(should_work_empty, MjAccordion, "<mj-accordion />");

    crate::should_not_sync_parse!(
        should_error_with_text,
        MjAccordion,
        "<mj-accordion>Hello</mj-accordion>",
        "UnexpectedToken(Span { start: 14, end: 19 })"
    );

    crate::should_not_sync_parse!(
        should_error_with_unknown_element,
        MjAccordion,
        "<mj-accordion><span /></mj-accordion>",
        "UnexpectedElement(Span { start: 14, end: 19 })"
    );
}
