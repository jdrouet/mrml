use htmlparser::StrSpan;

use super::MjAttributesChild;
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::NAME as MJ_CLASS;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseElement<MjAttributesChild> for MrmlParser<'_> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesChild, Error> {
        Ok(match tag.as_str() {
            MJ_ALL => MjAttributesChild::MjAttributesAll(self.parse(cursor, tag)?),
            MJ_CLASS => MjAttributesChild::MjAttributesClass(self.parse(cursor, tag)?),
            _ => MjAttributesChild::MjAttributesElement(self.parse(cursor, tag)?),
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjAttributesChild> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesChild, Error> {
        Ok(match tag.as_str() {
            MJ_ALL => MjAttributesChild::MjAttributesAll(self.async_parse(cursor, tag).await?),
            MJ_CLASS => MjAttributesChild::MjAttributesClass(self.async_parse(cursor, tag).await?),
            _ => MjAttributesChild::MjAttributesElement(self.async_parse(cursor, tag).await?),
        })
    }
}

impl ParseChildren<Vec<MjAttributesChild>> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjAttributesChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
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
impl AsyncParseChildren<Vec<MjAttributesChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjAttributesChild>, Error> {
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
    use crate::mj_attributes::MjAttributes;

    crate::should_sync_parse!(
        parse_complete,
        MjAttributes,
        r#"
    <mj-attributes>
        <mj-all color="red" />
        <mj-class name="head" color="green" />
        <span color="blue" />
    </mj-attributes>
"#
    );
}
