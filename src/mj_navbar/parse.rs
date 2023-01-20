use super::MJNavbarChild;
use crate::mj_navbar_link::{MJNavbarLink, NAME as MJ_NAVBAR_LINK};
use crate::prelude::parse::{Error, Parsable};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MJNavbarChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_NAVBAR_LINK => Ok(MJNavbarLink::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
