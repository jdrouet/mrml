use super::{MJHead, MJHeadChild};
use crate::mj_attributes::MJAttributes;
use crate::mj_attributes::NAME as MJ_ATTRIBUTES;
use crate::mj_breakpoint::MJBreakpoint;
use crate::mj_breakpoint::NAME as MJ_BREAKPOINT;
use crate::mj_font::MJFont;
use crate::mj_font::NAME as MJ_FONT;
use crate::mj_preview::MJPreview;
use crate::mj_preview::NAME as MJ_PREVIEW;
use crate::mj_style::MJStyle;
use crate::mj_style::NAME as MJ_STYLE;
use crate::mj_title::MJTitle;
use crate::mj_title::NAME as MJ_TITLE;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MJHeadChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => Ok(MJAttributes::parse(tag, tokenizer)?.into()),
            MJ_BREAKPOINT => Ok(MJBreakpoint::parse(tag, tokenizer)?.into()),
            MJ_FONT => Ok(MJFont::parse(tag, tokenizer)?.into()),
            MJ_PREVIEW => Ok(MJPreview::parse(tag, tokenizer)?.into()),
            MJ_STYLE => Ok(MJStyle::parse(tag, tokenizer)?.into()),
            MJ_TITLE => Ok(MJTitle::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}

#[derive(Debug, Default)]
struct MJHeadParser(MJHead);

impl Parser for MJHeadParser {
    type Output = MJHead;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.0.children.push(MJHeadChild::parse(tag, tokenizer)?);
        Ok(())
    }
}

impl Parsable for MJHead {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJHeadParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unexpected_element() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-text>Hello World!</mj-text></mj-head></mjml>"#,
        );
        assert!(res.is_err());
    }
}
