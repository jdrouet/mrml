use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjHead, MjHeadChild};
use crate::mj_attributes::{MjAttributes, NAME as MJ_ATTRIBUTES};
use crate::mj_breakpoint::{MjBreakpoint, NAME as MJ_BREAKPOINT};
use crate::mj_font::{MjFont, NAME as MJ_FONT};
use crate::mj_preview::{MjPreview, NAME as MJ_PREVIEW};
use crate::mj_raw::{MjRaw, NAME as MJ_RAW};
use crate::mj_style::{MjStyle, NAME as MJ_STYLE};
use crate::mj_title::{MjTitle, NAME as MJ_TITLE};
use crate::prelude::parser::{
    ChildrenParser, ElementEnd, ElementParser, Error, MrmlParser, Parsable, Parser, ParserOptions,
};

impl<'a> ChildrenParser<'a, Vec<MjHeadChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjHeadChild>, Error> {
        let mut children = Vec::new();
        while let Some(token) = self.iterate() {
            todo!()
        }
        Ok(children)
    }
}

impl<'a> ElementParser<'a, MjHead> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjHead, Error> {
        let ending = self.assert_next_as::<ElementEnd>()?;
        let children = if !ending.empty {
            self.parse_children()?
        } else {
            Default::default()
        };

        Ok(MjHead { children })
    }
}

impl Parsable for MjHeadChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => Ok(MjAttributes::parse(tag, tokenizer, opts)?.into()),
            MJ_BREAKPOINT => Ok(MjBreakpoint::parse(tag, tokenizer, opts)?.into()),
            MJ_FONT => Ok(MjFont::parse(tag, tokenizer, opts)?.into()),
            crate::mj_include::NAME => {
                Ok(crate::mj_include::head::MjIncludeHead::parse(tag, tokenizer, opts)?.into())
            }
            MJ_PREVIEW => Ok(MjPreview::parse(tag, tokenizer, opts)?.into()),
            MJ_RAW => Ok(MjRaw::parse(tag, tokenizer, opts)?.into()),
            MJ_STYLE => Ok(MjStyle::parse(tag, tokenizer, opts)?.into()),
            MJ_TITLE => Ok(MjTitle::parse(tag, tokenizer, opts)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}

#[derive(Debug)]
struct MjHeadParser {
    opts: Rc<ParserOptions>,
    children: Vec<MjHeadChild>,
}

impl MjHeadParser {
    fn new(opts: Rc<ParserOptions>) -> Self {
        Self {
            opts,
            children: Vec::default(),
        }
    }
}

impl Parser for MjHeadParser {
    type Output = MjHead;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MjHead {
            children: self.children,
        })
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.children
            .push(MjHeadChild::parse(tag, tokenizer, self.opts.clone())?);
        Ok(())
    }
}

impl Parsable for MjHead {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjHeadParser::new(opts).parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn raw_children() {
        let res = crate::mjml::Mjml::parse(
            r#"<mjml><mj-head><mj-raw>Hello World!</mj-raw></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }
    #[test]
    fn unexpected_element() {
        let res = crate::mjml::Mjml::parse(
            r#"<mjml><mj-head><mj-text>Hello World!</mj-text></mj-head></mjml>"#,
        );
        assert!(res.is_err());
    }
}
