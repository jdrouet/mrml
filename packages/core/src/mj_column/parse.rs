use super::MJColumn;
use crate::mj_body::MJBodyChild;
use crate::prelude::parse::{Error, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::Tokenizer;

#[derive(Debug, Default)]
struct MJColumnParser(MJColumn);

impl Parser for MJColumnParser {
    type Output = MJColumn;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJBodyChild);
    parse_comment!();
    parse_text!();
}

impl MJColumn {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJColumnParser::default().parse(tokenizer)?.build()
    }
}
