use super::MJBreakpoint;
use crate::prelude::parse::{Error, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJBreakpointParser(MJBreakpoint);

impl Parser for MJBreakpointParser {
    type Output = MJBreakpoint;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "value" {
            self.0.value = value.to_string();
            Ok(())
        } else {
            Err(Error::UnexpectedAttribute(name.start()))
        }
    }
}

impl MJBreakpoint {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJBreakpointParser::default().parse(tokenizer)?.build()
    }
}
