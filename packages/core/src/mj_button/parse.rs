use super::MJButton;
use crate::mj_body::MJBodyChild;
use crate::prelude::parse::{Error, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::Tokenizer;

#[derive(Debug, Default)]
struct MJButtonParser(MJButton);

impl Parser for MJButtonParser {
    type Output = MJButton;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJBodyChild);
    parse_comment!();
    parse_text!();
}

impl MJButton {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJButtonParser::default().parse(tokenizer)?.build()
    }
}
