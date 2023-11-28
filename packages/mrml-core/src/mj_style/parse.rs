use xmlparser::StrSpan;

use super::{MjStyle, MjStyleAttributes};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

impl ParseAttributes<MjStyleAttributes> for MrmlParser {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<MjStyleAttributes, Error> {
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
}

impl ParseElement<MjStyle> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _: StrSpan<'a>) -> Result<MjStyle, Error> {
        let attributes = self.parse_attributes(cursor)?;
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
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl crate::prelude::parser::AsyncParseElement<MjStyle> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _: StrSpan<'a>,
    ) -> Result<MjStyle, Error> {
        let attributes = self.parse_attributes(cursor)?;
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
}

#[cfg(test)]
mod tests {
    use crate::mj_style::MjStyle;
    use crate::prelude::parser::{MrmlCursor, MrmlParser};

    crate::should_parse!(should_work_empty, MjStyle, "<mj-style />");
    crate::should_parse!(
        should_work_inline,
        MjStyle,
        r#"<mj-style inline="inline" />"#
    );
    crate::should_parse!(
        should_work_basic,
        MjStyle,
        r#"<mj-style>.whatever {background-color: red};</mj-style>"#
    );

    #[test]
    #[should_panic(expected = "UnexpectedAttribute(Span { start: 10, end: 21 })")]
    fn should_error_with_unknown_attribute() {
        let template = r#"<mj-style oups="true">.whatever {background-color: red};</mj-style>"#;
        let parser = MrmlParser::default();
        let mut cursor = MrmlCursor::new(template);
        let _: MjStyle = parser.parse_root(&mut cursor).unwrap();
    }
}
