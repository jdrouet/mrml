use xmlparser::StrSpan;

use super::{MjHead, MjHeadChild};
use crate::mj_attributes::NAME as MJ_ATTRIBUTES;
use crate::mj_breakpoint::NAME as MJ_BREAKPOINT;
use crate::mj_font::NAME as MJ_FONT;
use crate::mj_include::NAME as MJ_INCLUDE;
use crate::mj_preview::NAME as MJ_PREVIEW;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_style::NAME as MJ_STYLE;
use crate::mj_title::NAME as MJ_TITLE;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<Vec<MjHeadChild>> for MrmlParser {
    fn parse_children<'a>(&self, cursor: &mut MrmlCursor<'a>) -> Result<Vec<MjHeadChild>, Error> {
        let mut result = Vec::new();
        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl crate::prelude::parser::AsyncParseChildren<Vec<MjHeadChild>> for MrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjHeadChild>, Error> {
        use crate::prelude::parser::AsyncParseElement;

        let mut result = Vec::new();
        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    result.push(self.async_parse(cursor, inner.local).await?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
    }
}

impl ParseElement<MjHead> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjHead, Error> {
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok(MjHead::default());
        }
        let children = self.parse_children(cursor)?;
        cursor.assert_element_close()?;

        Ok(MjHead { children })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl crate::prelude::parser::AsyncParseElement<MjHead> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjHead, Error> {
        use crate::prelude::parser::AsyncParseChildren;

        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok(MjHead::default());
        }
        let children = self.async_parse_children(cursor).await?;
        cursor.assert_element_close()?;

        Ok(MjHead { children })
    }
}

impl ParseElement<MjHeadChild> for MrmlParser {
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
            _ => Err(Error::UnexpectedElement(tag.into())),
        }
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl crate::prelude::parser::AsyncParseElement<MjHeadChild> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self.parse(cursor, tag).map(MjHeadChild::MjAttributes),
            MJ_BREAKPOINT => self.parse(cursor, tag).map(MjHeadChild::MjBreakpoint),
            MJ_FONT => self.parse(cursor, tag).map(MjHeadChild::MjFont),
            MJ_INCLUDE => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjInclude),
            MJ_PREVIEW => self.parse(cursor, tag).map(MjHeadChild::MjPreview),
            MJ_RAW => self.parse(cursor, tag).map(MjHeadChild::MjRaw),
            MJ_STYLE => self
                .async_parse(cursor, tag)
                .await
                .map(MjHeadChild::MjStyle),
            MJ_TITLE => self.parse(cursor, tag).map(MjHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement(tag.into())),
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

    crate::should_not_parse!(
        unexpected_element,
        MjHead,
        "<mj-head><mj-text>Hello World!</mj-text></mj-head>"
    );
}
