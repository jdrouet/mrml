use xmlparser::StrSpan;

use super::MjTitle;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjTitle, Error> {
    let ending = cursor.assert_element_end()?;
    if ending.empty {
        return Ok(MjTitle::default());
    }

    let text = cursor.next_text()?.map(|inner| inner.text.to_string());

    cursor.assert_element_close()?;

    Ok(MjTitle {
        children: text.unwrap_or_default(),
    })
}

impl<'opts> ParseElement<MjTitle> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _: StrSpan<'a>) -> Result<MjTitle, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjTitle> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _: StrSpan<'a>,
    ) -> Result<MjTitle, Error> {
        parse(cursor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_title::MjTitle;

    crate::should_sync_parse!(self_closing, MjTitle, "<mj-title />");
    crate::should_sync_parse!(normal, MjTitle, "<mj-title>Hello World!</mj-title>");
}
