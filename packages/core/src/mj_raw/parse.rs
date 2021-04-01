use super::{MJRaw, MJRawChild};
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJRawParser(MJRaw);

impl Parser for MJRawParser {
    type Output = MJRaw;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_child!(MJRawChild);
    parse_comment!();
    parse_text!();
}

impl Parsable for MJRaw {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJRawParser::default().parse(tokenizer)?.build()
    }
}
