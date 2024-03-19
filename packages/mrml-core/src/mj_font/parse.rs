use xmlparser::StrSpan;

use super::{MjFont, MjFontAttributes};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

#[inline]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjFontAttributes, Error> {
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

impl<'opts> ParseAttributes<MjFontAttributes> for MrmlParser<'opts> {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<MjFontAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjFontAttributes> for AsyncMrmlParser {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<MjFontAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjFont, Error> {
    let attributes = parse_attributes(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }

    Ok(MjFont { attributes })
}

impl<'opts> ParseElement<MjFont> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _: StrSpan<'a>) -> Result<MjFont, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjFont> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _: StrSpan<'a>,
    ) -> Result<MjFont, Error> {
        parse(cursor)
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

    crate::should_not_sync_parse!(
        unexpected_attribute,
        MjFont,
        r#"<mj-font unknown="whatever" />"#
    );
}
