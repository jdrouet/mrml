use super::MjBreakpointAttributes;
#[cfg(feature = "async")]
use crate::prelude::parser::AsyncMrmlParser;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes};

#[inline]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjBreakpointAttributes, Error> {
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

impl<'opts> ParseAttributes<MjBreakpointAttributes> for MrmlParser<'opts> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<MjBreakpointAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjBreakpointAttributes> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<MjBreakpointAttributes, Error> {
        parse_attributes(cursor)
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
