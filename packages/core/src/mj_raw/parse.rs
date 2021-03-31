use super::{MJRaw, MJRawChild};
use crate::prelude::parse::{Error, Parser};
use crate::{parse_child, parse_comment, parse_text};
use xmlparser::Tokenizer;

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

impl MJRaw {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJRawParser::default().parse(tokenizer)?.build()
    }
}
