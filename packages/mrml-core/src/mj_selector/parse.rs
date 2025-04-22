use htmlparser::StrSpan;

#[cfg(feature = "async")]
use super::MjSelector;
use super::MjSelectorAttributes;
use crate::mj_html_attribute::MjHtmlAttribute;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren, ParseElement,
    WarningKind,
};

#[inline(always)]
fn parse_attributes(
    cursor: &mut MrmlCursor<'_>,
    tag: &StrSpan<'_>,
) -> Result<MjSelectorAttributes, Error> {
    let mut path = None;

    while let Some(attr) = cursor.next_attribute()? {
        match (attr.local.as_str(), attr.value) {
            ("path", Some(value)) => {
                path = Some(value.to_string());
            }
            _ => {
                cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
            }
        }
    }

    Ok(MjSelectorAttributes {
        path: path.ok_or_else(|| Error::MissingAttribute {
            name: "path",
            origin: cursor.origin(),
            position: tag.into(),
        })?,
    })
}

impl ParseAttributes<MjSelectorAttributes> for MrmlParser<'_> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        tag: &StrSpan<'_>,
    ) -> Result<MjSelectorAttributes, Error> {
        parse_attributes(cursor, tag)
    }
}

impl ParseChildren<Vec<MjHtmlAttribute>> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjHtmlAttribute>, Error> {
        let mut children = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    children.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
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
impl AsyncParseChildren<Vec<MjHtmlAttribute>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjHtmlAttribute>, Error> {
        let mut children = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    children.push(self.async_parse(cursor, inner.local).await?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
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
impl AsyncParseElement<MjSelector> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjSelector, Error> {
        let attributes = parse_attributes(cursor, &tag)?;

        let children = self.async_parse_children(cursor).await?;
        cursor.assert_element_close()?;

        Ok(MjSelector::new(attributes, children))
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_selector::MjSelector;

    crate::should_sync_parse!(
        parse_complete,
        MjSelector,
        r#"<mj-selector path="data-id" />"#
    );

    crate::should_not_sync_parse!(
        should_have_name,
        MjSelector,
        r#"<mj-selector />"#,
        r#"MissingAttribute { name: "path", origin: Root, position: Span { start: 1, end: 12 } }"#
    );
}
