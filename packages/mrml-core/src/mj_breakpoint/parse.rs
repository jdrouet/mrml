use xmlparser::StrSpan;

use super::{MjBreakpoint, MjBreakpointAttributes};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

impl ParseAttributes<MjBreakpointAttributes> for MrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<MjBreakpointAttributes, Error> {
        let mut result = MjBreakpointAttributes::default();
        while let Some(attr) = cursor.next_attribute()? {
            if attr.local.as_str() == "width" {
                result.width = attr.value.to_string();
            } else {
                return Err(Error::UnexpectedAttribute(attr.span.into()));
            }
        }
        Ok(result)
    }
}

impl ParseElement<MjBreakpoint> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjBreakpoint, Error> {
        let attributes = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            return Err(Error::InvalidFormat(ending.span.into()));
        }

        Ok(MjBreakpoint { attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::MjBreakpoint;

    crate::should_sync_parse!(success, MjBreakpoint, r#"<mj-breakpoint width="42px" />"#);
    crate::should_not_sync_parse!(
        unexpected_attributes,
        MjBreakpoint,
        r#"<mj-breakpoint whatever="42px" />"#
    );
}
