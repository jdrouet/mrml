use xmlparser::StrSpan;

use super::MjAttributesClass;
use crate::prelude::hash::Map;
use crate::prelude::parser::{parse_attributes_map, Error, MrmlCursor, MrmlParser, ParseElement};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};

#[inline]
fn parse<'a>(cursor: &mut MrmlCursor<'a>, tag: StrSpan<'a>) -> Result<MjAttributesClass, Error> {
    let mut attributes: Map<String, String> = parse_attributes_map(cursor)?;
    let name: String = attributes
        .remove("name")
        .ok_or_else(|| Error::MissingAttribute("name", tag.into()))?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }

    Ok(MjAttributesClass { name, attributes })
}

impl<'opts> ParseElement<MjAttributesClass> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesClass, Error> {
        parse(cursor, tag)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjAttributesClass> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesClass, Error> {
        parse(cursor, tag)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes_class::MjAttributesClass;

    crate::should_sync_parse!(
        parse_complete,
        MjAttributesClass,
        r#"<mj-class name="whatever" color="red" />"#
    );
    crate::should_not_sync_parse!(
        should_have_name,
        MjAttributesClass,
        r#"<mj-class color="red" />"#,
        "MissingAttribute(\"name\", Span { start: 1, end: 9 })"
    );
    crate::should_not_sync_parse!(
        should_close,
        MjAttributesClass,
        r#"<mj-class name="div" color="red"><whatever>"#,
        "UnexpectedToken(Span { start: 33, end: 42 })"
    );
}
