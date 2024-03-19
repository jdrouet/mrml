use xmlparser::StrSpan;

use super::{MjBreakpoint, MjBreakpointAttributes};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

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

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjBreakpoint, Error> {
    let attributes = parse_attributes(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        return Err(Error::InvalidFormat(ending.span.into()));
    }

    Ok(MjBreakpoint { attributes })
}

impl<'opts> ParseElement<MjBreakpoint> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjBreakpoint, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjBreakpoint> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjBreakpoint, Error> {
        parse(cursor)
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
