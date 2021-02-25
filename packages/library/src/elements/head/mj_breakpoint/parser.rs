use super::MJBreakpoint;
use crate::parser::{Error, MJMLParser};
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
            Err(Error::InvalidElement(
                "mj-breakpoint expects a width attribute".into(),
            ))
        }
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "width" {
            self.width = Some(value.as_str().parse::<Size>().map_err(|_err| {
                Error::InvalidElement(format!(
                    "mj-breakpoint width attribute is invalid at position {}",
                    value.start()
                ))
            })?);
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
