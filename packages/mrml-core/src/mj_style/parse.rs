use super::MjStyleAttributes;
#[cfg(feature = "async")]
use crate::prelude::parser::AsyncMrmlParser;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, WarningKind};

#[inline(always)]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjStyleAttributes, Error> {
    let mut result = MjStyleAttributes::default();
    while let Some(attr) = cursor.next_attribute()? {
        if attr.local.as_str() == "inline" {
            result.inline = Some(attr.value.to_string());
        } else {
            cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
        }
    }
    Ok(result)
}

impl<'opts> ParseAttributes<MjStyleAttributes> for MrmlParser<'opts> {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<MjStyleAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjStyleAttributes> for AsyncMrmlParser {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<MjStyleAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_style::MjStyle;
    use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions};

    crate::should_sync_parse!(should_work_empty, MjStyle, "<mj-style />");
    crate::should_sync_parse!(
        should_work_inline,
        MjStyle,
        r#"<mj-style inline="inline" />"#
    );
    crate::should_sync_parse!(
        should_work_basic,
        MjStyle,
        r#"<mj-style>.whatever {background-color: red};</mj-style>"#
    );

    #[test]
    fn should_warn_with_unknown_attribute() {
        let template = r#"<mj-style oups="true">.whatever {background-color: red};</mj-style>"#;
        let opts = ParserOptions::default();
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(template);
        let _: MjStyle = parser.parse_root(&mut cursor).unwrap();
        assert_eq!(cursor.warnings().len(), 1);
    }
}
