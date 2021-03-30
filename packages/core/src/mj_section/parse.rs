use super::MJSection;
use crate::mj_body::MJBodyChild;
use crate::prelude::parse::{Error, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::Tokenizer;

#[derive(Debug, Default)]
struct MJSectionParser(MJSection);

impl Parser for MJSectionParser {
    type Output = MJSection;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJBodyChild);
    parse_comment!();
    parse_text!();
}

impl MJSection {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJSectionParser::default().parse(tokenizer)?.build()
    }
}
