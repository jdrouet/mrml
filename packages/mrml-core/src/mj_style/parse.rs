use xmlparser::StrSpan;

use super::{MjStyle, MjStyleAttributes};
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};

impl<'a> AttributesParser<'a, MjStyleAttributes> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<MjStyleAttributes, Error> {
        let mut result = MjStyleAttributes::default();
        while let Some(attr) = self.next_attribute()? {
            if attr.local.as_str() == "inline" {
                result.inline = Some(attr.value.to_string());
            } else {
                return Err(Error::UnexpectedAttribute(attr.span.into()));
            }
        }
        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjStyle> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjStyle, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            let children = self
                .next_text()?
                .map(|txt| txt.text.to_string())
                .unwrap_or_default();
            self.assert_element_close()?;

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
    use crate::{mj_style::MjStyle, prelude::parser::MrmlParser};

    #[test]
    fn should_work_empty() {
        let _: MjStyle = MrmlParser::new(r#"<mj-style />"#, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn should_work_inline() {
        let _: MjStyle = MrmlParser::new(r#"<mj-style inline="inline" />"#, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn should_work_basic() {
        let _: MjStyle = MrmlParser::new(
            r#"<mj-style>.whatever {background-color: red};</mj-style>"#,
            Default::default(),
        )
        .parse_root()
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedAttribute(Span { start: 10, end: 21 })")]
    fn should_error_with_unknown_attribute() {
        let _: MjStyle = MrmlParser::new(
            r#"<mj-style oups="true">.whatever {background-color: red};</mj-style>"#,
            Default::default(),
        )
        .parse_root()
        .unwrap();
    }
}
