use super::MjSocialChild;
use crate::mj_social_element::{MjSocialElement, NAME as MJ_SOCIAL_ELEMENT};
use crate::prelude::parse::{Error, Parsable};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjSocialChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_SOCIAL_ELEMENT => Ok(MjSocialElement::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
