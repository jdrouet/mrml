use xmlparser::StrSpan;

use super::{MjStyle, MjStyleAttributes};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

#[inline]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjStyleAttributes, Error> {
    let mut result = MjStyleAttributes::default();
    while let Some(attr) = cursor.next_attribute()? {
        if attr.local.as_str() == "inline" {
            result.inline = Some(attr.value.to_string());
        } else {
            return Err(Error::UnexpectedAttribute(attr.span.into()));
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

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjStyle, Error> {
    let attributes = parse_attributes(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        let children = cursor
            .next_text()?
            .map(|txt| txt.text.to_string())
            .unwrap_or_default();
        cursor.assert_element_close()?;

        Ok(MjStyle {
            attributes,
            children,
        })
    } else {
        Ok(MjStyle {
            attributes,
            children: String::new(),
        })
    }
}

impl<'opts> ParseElement<MjStyle> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _: StrSpan<'a>) -> Result<MjStyle, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjStyle> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _: StrSpan<'a>,
    ) -> Result<MjStyle, Error> {
        parse(cursor)
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
    #[should_panic(expected = "UnexpectedAttribute(Span { start: 10, end: 21 })")]
    fn should_error_with_unknown_attribute() {
        let template = r#"<mj-style oups="true">.whatever {background-color: red};</mj-style>"#;
        let opts = ParserOptions::default();
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(template);
        let _: MjStyle = parser.parse_root(&mut cursor).unwrap();
    }
}
