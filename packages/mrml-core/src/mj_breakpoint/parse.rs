use xmlparser::StrSpan;

use super::{MjBreakpoint, MjBreakpointAttributes};
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};

impl<'a> AttributesParser<'a, MjBreakpointAttributes> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<MjBreakpointAttributes, Error> {
        let mut result = MjBreakpointAttributes::default();
        while let Some(attr) = self.next_attribute()? {
            if attr.local.as_str() == "width" {
                result.width = attr.value.to_string();
            } else {
                return Err(Error::UnexpectedAttribute(attr.span.into()));
            }
        }
        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjBreakpoint> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjBreakpoint, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            return Err(Error::InvalidFormat(ending.span.into()));
        }

        Ok(MjBreakpoint { attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::MjBreakpoint;
    use crate::prelude::parser::MrmlParser;

    #[test]
    fn success() {
        let raw = r#"<mj-breakpoint width="42px" />"#;
        let _: MjBreakpoint = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn unexpected_attributes() {
        let raw = r#"<mj-breakpoint whatever="42px" />"#;
        let _: MjBreakpoint = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
