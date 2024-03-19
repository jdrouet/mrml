use xmlparser::StrSpan;

use super::MjPreview;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjPreview, Error> {
    let ending = cursor.assert_element_end()?;
    if ending.empty {
        return Ok(MjPreview::default());
    }

    let text = cursor.next_text()?.map(|inner| inner.text.to_string());

    cursor.assert_element_close()?;

    Ok(MjPreview {
        children: text.unwrap_or_default(),
    })
}

impl<'opts> ParseElement<MjPreview> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjPreview, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjPreview> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjPreview, Error> {
        parse(cursor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_preview::MjPreview;

    crate::should_sync_parse!(
        should_parse,
        MjPreview,
        "<mj-preview>Hello World!</mj-preview>"
    );
    crate::should_sync_parse!(should_parse_without_children, MjPreview, "<mj-preview />");
}
