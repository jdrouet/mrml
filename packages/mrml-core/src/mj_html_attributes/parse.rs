use super::MjSelector;
use crate::mj_selector::NAME as MJ_SELECTOR;

#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<Vec<MjSelector>> for MrmlParser<'_> {
    fn parse_children<'a>(&self, cursor: &mut MrmlCursor<'a>) -> Result<Vec<MjSelector>, Error> {
        let mut children = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_SELECTOR {
                        children.push(self.parse(cursor, inner.local)?)
                    } else {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: inner.span.into(),
                        });
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(children);
                }
                other => {
                    return Err(Error::UnexpectedElement {
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
impl AsyncParseChildren<Vec<MjSelector>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjSelector>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    result.push(self.async_parse(cursor, inner.local).await?);
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
    use crate::mj_html_attributes::MjHtmlAttributes;

    crate::should_sync_parse!(
        parse_complete,
        MjHtmlAttributes,
        r#"
    <mj-html-attributes>
        <mj-selector path=".custom div">
            <mj-html-attribute name="data-id">43</mj-html-attribute>
        </mj-selector>
    </mj-html-attributes>
"#
    );
}
