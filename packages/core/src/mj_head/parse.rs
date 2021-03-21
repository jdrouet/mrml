use super::{MJHead, MJHeadChild};
use crate::prelude::parse::{Error, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJHeadParser(MJHead);

impl Parser for MJHeadParser {
    type Output = MJHead;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.0.children.push(MJHeadChild::parse(tag, tokenizer)?);
        Ok(())
    }
}

impl MJHead {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJHeadParser::default().parse(tokenizer)?.build()
    }
}
