use super::MJText;
use crate::mj_body::MJBodyChild;
use crate::prelude::parse::{Error, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::Tokenizer;

#[derive(Debug, Default)]
struct MJTextParser(MJText);

impl Parser for MJTextParser {
    type Output = MJText;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJBodyChild);
    parse_comment!();
    parse_text!();
}

impl MJText {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJTextParser::default().parse(tokenizer)?.build()
    }
}
