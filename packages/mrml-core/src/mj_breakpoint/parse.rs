use xmlparser::StrSpan;

use super::MjBreakpointAttributes;
#[cfg(feature = "async")]
use crate::prelude::parser::AsyncMrmlParser;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, WarningKind};

#[inline]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjBreakpointAttributes, Error> {
    let mut result = MjBreakpointAttributes::default();
    while let Some(attr) = cursor.next_attribute()? {
        if attr.local.as_str() == "width" {
            result.width = attr.value.to_string();
        } else {
            cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
        }
    }
    Ok(result)
}

impl<'opts> ParseAttributes<MjBreakpointAttributes> for MrmlParser<'opts> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<MjBreakpointAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjBreakpointAttributes> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<MjBreakpointAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::MjBreakpoint;

    crate::should_sync_parse!(
        success,
        MjBreakpoint,
        r#"<mj-breakpoint width="42px" />"#,
        0
    );
    crate::should_sync_parse!(
        unexpected_attributes,
        MjBreakpoint,
        r#"<mj-breakpoint whatever="42px" />"#,
        1
    );
}
