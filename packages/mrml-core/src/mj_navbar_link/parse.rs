use super::{MJNavbarLink, MJNavbarLinkChild};
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MJNavbarLinkChild {
    fn parse<'a>(tag: StrSpan<'a>, _tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        Err(Error::UnexpectedElement(tag.start()))
    }
}

#[derive(Debug, Default)]
struct MJNavbarLinkParser(MJNavbarLink);

impl Parser for MJNavbarLinkParser {
    type Output = MJNavbarLink;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJNavbarLinkChild);
    parse_comment!();
    parse_text!();
}

impl Parsable for MJNavbarLink {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJNavbarLinkParser::default().parse(tokenizer)?.build()
    }
}
