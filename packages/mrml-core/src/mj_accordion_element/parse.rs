use super::MjAccordionElementChildren;
use crate::mj_accordion_text::NAME as MJ_ACCORDION_TEXT;
use crate::mj_accordion_title::NAME as MJ_ACCORDION_TITLE;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<MjAccordionElementChildren> for MrmlParser<'_> {
    fn parse_children(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<MjAccordionElementChildren, Error> {
        let mut result = MjAccordionElementChildren::default();

        loop {
            let token = cursor.assert_next()?;
            match token {
                MrmlToken::ElementStart(inner) => match inner.local.as_str() {
                    MJ_ACCORDION_TEXT => {
                        result.text = Some(self.parse(cursor, inner.local)?);
                    }
                    MJ_ACCORDION_TITLE => {
                        result.title = Some(self.parse(cursor, inner.local)?);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: inner.span.into(),
                        });
                    }
                },
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    });
                }
            }
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<MjAccordionElementChildren> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<MjAccordionElementChildren, Error> {
        let mut result = MjAccordionElementChildren::default();

        loop {
            let token = cursor.assert_next()?;
            match token {
                MrmlToken::ElementStart(inner) => match inner.local.as_str() {
                    MJ_ACCORDION_TEXT => {
                        result.text = Some(self.async_parse(cursor, inner.local).await?);
                    }
                    MJ_ACCORDION_TITLE => {
                        result.title = Some(self.async_parse(cursor, inner.local).await?);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: inner.span.into(),
                        });
                    }
                },
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion_element::MjAccordionElement;

    crate::should_sync_parse!(
        should_work_with_no_children,
        MjAccordionElement,
        "<mj-accordion-element />"
    );

    crate::should_not_sync_parse!(
        should_error_with_unknown_child,
        MjAccordionElement,
        "<mj-accordion-element><span /></mj-accordion-element>",
        "UnexpectedElement { origin: Root, position: Span { start: 22, end: 27 } }"
    );

    crate::should_not_sync_parse!(
        should_error_with_comment,
        MjAccordionElement,
        "<mj-accordion-element><!-- comment --></mj-accordion-element>",
        "UnexpectedToken { origin: Root, position: Span { start: 22, end: 38 } }"
    );
}
