use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::MjSocialChild;
use crate::mj_social_element::{MjSocialElement, NAME as MJ_SOCIAL_ELEMENT};
use crate::prelude::parser::{Error, Parsable, ParserOptions};

impl Parsable for MjSocialChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_SOCIAL_ELEMENT => Ok(MjSocialElement::parse(tag, tokenizer, opts)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
