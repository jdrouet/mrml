use super::MJSocial;
use super::MJSocialChild;
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
