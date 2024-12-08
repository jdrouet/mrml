use htmlparser::StrSpan;

use super::MjHeadChild;
use crate::comment::Comment;
use crate::mj_attributes::NAME as MJ_ATTRIBUTES;
use crate::mj_breakpoint::NAME as MJ_BREAKPOINT;
use crate::mj_font::NAME as MJ_FONT;
use crate::mj_include::NAME as MJ_INCLUDE;
use crate::mj_preview::NAME as MJ_PREVIEW;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_style::NAME as MJ_STYLE;
use crate::mj_title::NAME as MJ_TITLE;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<Vec<MjHeadChild>> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjHeadChild>, Error> {
        let mut result = Vec::new();
        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjHeadChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
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
impl AsyncParseChildren<Vec<MjHeadChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjHeadChild>, Error> {
        use crate::prelude::parser::AsyncParseElement;

        let mut result = Vec::new();
        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjHeadChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.async_parse(cursor, inner.local).await?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
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

impl ParseElement<MjHeadChild> for MrmlParser<'_> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self.parse(cursor, tag).map(MjHeadChild::MjAttributes),
            MJ_BREAKPOINT => self.parse(cursor, tag).map(MjHeadChild::MjBreakpoint),
            MJ_FONT => self.parse(cursor, tag).map(MjHeadChild::MjFont),
            MJ_INCLUDE => self.parse(cursor, tag).map(MjHeadChild::MjInclude),
            MJ_PREVIEW => self.parse(cursor, tag).map(MjHeadChild::MjPreview),
            MJ_RAW => self.parse(cursor, tag).map(MjHeadChild::MjRaw),
            MJ_STYLE => self.parse(cursor, tag).map(MjHeadChild::MjStyle),
            MJ_TITLE => self.parse(cursor, tag).map(MjHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement {
                origin: cursor.origin(),
                position: tag.into(),
            }),
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjHeadChild> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjAttributes),
            MJ_BREAKPOINT => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjBreakpoint),
            MJ_FONT => self.async_parse(cursor, tag).await.map(MjHeadChild::MjFont),
            MJ_INCLUDE => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjInclude),
            MJ_PREVIEW => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjPreview),
            MJ_RAW => self.async_parse(cursor, tag).await.map(MjHeadChild::MjRaw),
            MJ_STYLE => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjStyle),
            MJ_TITLE => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement {
                origin: cursor.origin(),
                position: tag.into(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_head::MjHead;

    crate::should_parse!(
        raw_children,
        MjHead,
        "<mj-head><mj-raw>Hello World!</mj-raw></mj-head>"
    );

    crate::should_parse!(with_comment, MjHead, "<mj-head><!-- HEAD --></mj-head>");

    crate::should_not_parse!(
        unexpected_element,
        MjHead,
        "<mj-head><mj-text>Hello World!</mj-text></mj-head>"
    );
}
