use super::{MJSocialElement, MJSocialElementChild};
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJSocialElementParser(MJSocialElement);

impl Parser for MJSocialElementParser {
    type Output = MJSocialElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_comment!();
    parse_child!(MJSocialElementChild);
    parse_text!();
}

impl Parsable for MJSocialElement {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJSocialElementParser::default().parse(tokenizer)?.build()
    }
}
