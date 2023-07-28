use xmlparser::StrSpan;

use super::{MjFont, MjFontAttributes};
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};

impl<'a> AttributesParser<'a, MjFontAttributes> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<MjFontAttributes, Error> {
        let mut result = MjFontAttributes::default();

        while let Some(attrs) = self.next_attribute()? {
            match attrs.local.as_str() {
                "name" => result.name = attrs.value.to_string(),
                "href" => result.href = attrs.value.to_string(),
                _ => return Err(Error::UnexpectedAttribute(attrs.span.start())),
            }
        }

        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjFont> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjFont, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            return Err(Error::InvalidFormat(ending.span.start(), ending.span.end()));
        }

        Ok(MjFont { attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::{mj_font::MjFont, prelude::parser::MrmlParser};

    #[test]
    fn success() {
        let _: MjFont = MrmlParser::new(
            r#"<mj-font name="Comic" href="https://jolimail.io" />"#,
            Default::default(),
        )
        .parse_root()
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn unexpected_attribute() {
        let _: MjFont = MrmlParser::new(r#"<mj-font unknown="whatever" />"#, Default::default())
            .parse_root()
            .unwrap();
    }
}
