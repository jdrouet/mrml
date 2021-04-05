use super::MJWrapper;
use crate::mj_body::MJBodyChild;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJWrapperParser(MJWrapper);

impl Parser for MJWrapperParser {
    type Output = MJWrapper;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJBodyChild);
    parse_comment!();
    parse_text!();
}

impl Parsable for MJWrapper {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJWrapperParser::default().parse(tokenizer)?.build()
    }
}
