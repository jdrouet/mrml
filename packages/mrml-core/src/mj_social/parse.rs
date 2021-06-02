use super::MJSocial;
use super::MJSocialChild;
use crate::mj_social_element::{MJSocialElement, NAME as MJ_SOCIAL_ELEMENT};
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJSocialParser(MJSocial);

impl Parser for MJSocialParser {
    type Output = MJSocial;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJSocialChild);
    parse_comment!();
}

impl Parsable for MJSocial {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJSocialParser::default().parse(tokenizer)?.build()
    }
}

impl Parsable for MJSocialChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_SOCIAL_ELEMENT => Ok(MJSocialElement::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
