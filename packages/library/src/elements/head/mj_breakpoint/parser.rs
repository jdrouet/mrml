use super::MJBreakpoint;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::size::Size;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Default)]
struct MJBreakpointParser {
    width: Option<Size>,
}

impl MJMLParser for MJBreakpointParser {
    type Output = MJBreakpoint;

    fn build(self) -> Result<Self::Output, Error> {
        if let Some(value) = self.width {
            Ok(MJBreakpoint { value })
        } else {
            Err(Error::MissingAttribute("expected width attribute".into()))
        }
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "width" {
            self.width = Some(value.as_str().parse::<Size>().map_err(|_err| {
                Error::UnexpectedAttribute("unable to parse width attribute".into())
            })?);
            Ok(())
        } else {
            Err(Error::UnexpectedAttribute(name.to_string()))
        }
    }
}

impl MJBreakpoint {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJBreakpointParser::default().parse(tokenizer)?.build()
    }
}
