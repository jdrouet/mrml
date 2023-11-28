use xmlparser::StrSpan;

use super::{MjFont, MjFontAttributes};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

impl ParseAttributes<MjFontAttributes> for MrmlParser {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<MjFontAttributes, Error> {
        let mut result = MjFontAttributes::default();

        while let Some(attrs) = cursor.next_attribute()? {
            match attrs.local.as_str() {
                "name" => result.name = attrs.value.to_string(),
                "href" => result.href = attrs.value.to_string(),
                _ => return Err(Error::UnexpectedAttribute(attrs.span.into())),
            }
        }

        Ok(result)
    }
}

impl ParseElement<MjFont> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _: StrSpan<'a>) -> Result<MjFont, Error> {
        let attributes = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            return Err(Error::InvalidFormat(ending.span.into()));
        }

        Ok(MjFont { attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_font::MjFont;

    crate::should_parse!(
        success,
        MjFont,
        r#"<mj-font name="Comic" href="https://jolimail.io" />"#
    );

    crate::should_not_parse!(
        unexpected_attribute,
        MjFont,
        r#"<mj-font unknown="whatever" />"#
    );
}
