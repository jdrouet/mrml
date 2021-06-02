use super::MJNavbar;
use super::MJNavbarChild;
use crate::mj_navbar_link::{MJNavbarLink, NAME as MJ_NAVBAR_LINK};
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MJNavbarChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_NAVBAR_LINK => Ok(MJNavbarLink::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}

#[derive(Debug, Default)]
struct MJNavbarParser(MJNavbar);

impl Parser for MJNavbarParser {
    type Output = MJNavbar;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJNavbarChild);
    parse_comment!();
}

impl Parsable for MJNavbar {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJNavbarParser::default().parse(tokenizer)?.build()
    }
}
