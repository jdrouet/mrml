use super::MjAccordionChild;
use crate::comment::Comment;
use crate::mj_accordion_element::NAME as MJ_ACCORDION_ELEMENT;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<Vec<MjAccordionChild>> for MrmlParser<'_> {
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
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: inner.span.into(),
                        });
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    })
                }
            }
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjAccordionChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjAccordionChild>, Error> {
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
                            self.async_parse(cursor, inner.local).await?,
                        ));
                    } else {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: inner.span.into(),
                        });
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion::MjAccordion;
    use crate::mjml::Mjml;
    use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions};

    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-accordion.mjml");
        let mut cursor = MrmlCursor::new(template);
        let opts = ParserOptions::default();
        let result: Mjml = MrmlParser::new(&opts).parse_root(&mut cursor).unwrap();
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
        "UnexpectedToken { origin: Root, position: Span { start: 14, end: 19 } }"
    );

    crate::should_not_sync_parse!(
        should_error_with_unknown_element,
        MjAccordion,
        "<mj-accordion><span /></mj-accordion>",
        "UnexpectedElement { origin: Root, position: Span { start: 14, end: 19 } }"
    );
}
