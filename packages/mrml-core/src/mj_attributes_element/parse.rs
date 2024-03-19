use xmlparser::StrSpan;

use super::MjAttributesElement;
use crate::prelude::hash::Map;
use crate::prelude::parser::{parse_attributes_map, Error, MrmlCursor, MrmlParser, ParseElement};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};

#[inline]
fn parse<'a>(cursor: &mut MrmlCursor<'a>, tag: StrSpan<'a>) -> Result<MjAttributesElement, Error> {
    let attributes: Map<String, String> = parse_attributes_map(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }

    Ok(MjAttributesElement {
        name: tag.to_string(),
        attributes,
    })
}

impl<'opts> ParseElement<MjAttributesElement> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesElement, Error> {
        parse(cursor, tag)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjAttributesElement> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesElement, Error> {
        parse(cursor, tag)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes::MjAttributes;

    crate::should_sync_parse!(
        parse_complete,
        MjAttributes,
        r#"<mj-attributes><mj-class name="whatever" color="red" /></mj-attributes>"#
    );
}
