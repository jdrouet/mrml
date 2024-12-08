use htmlparser::StrSpan;

use super::MjFontAttributes;
#[cfg(feature = "async")]
use crate::prelude::parser::AsyncMrmlParser;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, WarningKind};

#[inline(always)]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjFontAttributes, Error> {
    let mut result = MjFontAttributes::default();

    while let Some(attrs) = cursor.next_attribute()? {
        match (attrs.local.as_str(), attrs.value) {
            ("name", Some(value)) => result.name = value.to_string(),
            ("href", Some(value)) => result.href = value.to_string(),
            _ => cursor.add_warning(WarningKind::UnexpectedAttribute, attrs.span),
        }
    }

    Ok(result)
}

impl ParseAttributes<MjFontAttributes> for MrmlParser<'_> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<MjFontAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjFontAttributes> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<MjFontAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_font::MjFont;

    crate::should_sync_parse!(
        success,
        MjFont,
        r#"<mj-font name="Comic" href="https://jolimail.io" />"#
    );

    crate::should_sync_parse!(
        unexpected_attribute,
        MjFont,
        r#"<mj-font unknown="whatever" />"#,
        1
    );
}
