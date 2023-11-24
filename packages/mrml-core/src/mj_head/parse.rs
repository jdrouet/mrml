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
use crate::prelude::parser::{ChildrenParser, ElementParser, Error, MrmlCursor, MrmlToken};

impl<'a> ChildrenParser<'a, Vec<MjHeadChild>> for MrmlCursor<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjHeadChild>, Error> {
        let mut result = Vec::new();
        loop {
            match self.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    self.rewind(MrmlToken::ElementClose(close));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
    }
}

impl<'a> ElementParser<'a, MjHead> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjHead, Error> {
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjHead::default());
        }
        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjHead { children })
    }
}

impl<'a> ElementParser<'a, MjHeadChild> for MrmlCursor<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self.parse(tag).map(MjHeadChild::MjAttributes),
            MJ_BREAKPOINT => self.parse(tag).map(MjHeadChild::MjBreakpoint),
            MJ_FONT => self.parse(tag).map(MjHeadChild::MjFont),
            MJ_INCLUDE => self.parse(tag).map(MjHeadChild::MjInclude),
            MJ_PREVIEW => self.parse(tag).map(MjHeadChild::MjPreview),
            MJ_RAW => self.parse(tag).map(MjHeadChild::MjRaw),
            MJ_STYLE => self.parse(tag).map(MjHeadChild::MjStyle),
            MJ_TITLE => self.parse(tag).map(MjHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement(tag.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_head::MjHead;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn raw_children() {
        let _: MjHead = MrmlCursor::new(
            r#"<mj-head><mj-raw>Hello World!</mj-raw></mj-head>"#,
            Default::default(),
        )
        .parse_root()
        .unwrap();
    }
    #[test]
    #[should_panic]
    fn unexpected_element() {
        let item: MjHead = MrmlCursor::new(
            r#"<mj-head><mj-text>Hello World!</mj-text></mj-head>"#,
            Default::default(),
        )
        .parse_root()
        .unwrap();
        println!("{item:?}");
    }
}
