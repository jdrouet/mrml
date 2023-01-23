use super::{MjHead, MjHeadChild};
use crate::mj_attributes::MjAttributes;
use crate::mj_attributes::NAME as MJ_ATTRIBUTES;
use crate::mj_breakpoint::MjBreakpoint;
use crate::mj_breakpoint::NAME as MJ_BREAKPOINT;
use crate::mj_font::MjFont;
use crate::mj_font::NAME as MJ_FONT;
use crate::mj_preview::MjPreview;
use crate::mj_preview::NAME as MJ_PREVIEW;
use crate::mj_raw::MjRaw;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_style::MjStyle;
use crate::mj_style::NAME as MJ_STYLE;
use crate::mj_title::MjTitle;
use crate::mj_title::NAME as MJ_TITLE;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjHeadChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => Ok(MjAttributes::parse(tag, tokenizer)?.into()),
            MJ_BREAKPOINT => Ok(MjBreakpoint::parse(tag, tokenizer)?.into()),
            MJ_FONT => Ok(MjFont::parse(tag, tokenizer)?.into()),
            MJ_PREVIEW => Ok(MjPreview::parse(tag, tokenizer)?.into()),
            MJ_RAW => Ok(MjRaw::parse(tag, tokenizer)?.into()),
            MJ_STYLE => Ok(MjStyle::parse(tag, tokenizer)?.into()),
            MJ_TITLE => Ok(MjTitle::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}

#[derive(Debug, Default)]
struct MjHeadParser(MjHead);

impl Parser for MjHeadParser {
    type Output = MjHead;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.0.children.push(MjHeadChild::parse(tag, tokenizer)?);
        Ok(())
    }
}

impl Parsable for MjHead {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MjHeadParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn raw_children() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-raw>Hello World!</mj-raw></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }
    #[test]
    fn unexpected_element() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-text>Hello World!</mj-text></mj-head></mjml>"#,
        );
        assert!(res.is_err());
    }
}
