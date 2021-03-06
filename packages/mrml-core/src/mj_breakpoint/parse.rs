use super::{MJBreakpoint, MJBreakpointAttributes};
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJBreakpointParser(MJBreakpointAttributes);

impl Parser for MJBreakpointParser {
    type Output = MJBreakpoint;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJBreakpoint { attributes: self.0 })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "width" {
            self.0.width = value.to_string();
            Ok(())
        } else {
            Err(Error::UnexpectedAttribute(name.start()))
        }
    }
}

impl Parsable for MJBreakpoint {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJBreakpointParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-breakpoint width="42px" /></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn unexpected_attributes() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-breakpoint whatever="42px" /></mj-head></mjml>"#,
        );
        assert!(res.is_err());
    }
}
