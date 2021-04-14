use super::MJTitle;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJTitleParser(MJTitle);

impl Parser for MJTitleParser {
    type Output = MJTitle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.0.children = value.to_string();
        Ok(())
    }
}

impl Parsable for MJTitle {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJTitleParser::default().parse(tokenizer)?.build()
    }
}
