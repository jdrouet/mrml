use xmlparser::StrSpan;

use super::MjAttributesAll;
use crate::prelude::parser::{parse_attributes_map, Error, MrmlCursor, MrmlParser, ParseElement};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjAttributesAll, Error> {
    let attributes = parse_attributes_map(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }

    Ok(MjAttributesAll { attributes })
}

impl<'opts> ParseElement<MjAttributesAll> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAttributesAll, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjAttributesAll> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAttributesAll, Error> {
        parse(cursor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes_all::MjAttributesAll;

    crate::should_sync_parse!(parse_complete, MjAttributesAll, r#"<mj-all color="red" />"#);
}
