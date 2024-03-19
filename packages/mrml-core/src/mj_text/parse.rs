use xmlparser::StrSpan;

use super::MjText;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjText> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjText, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjText {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjText> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjText, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjText {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_text::MjText;

    crate::should_parse!(self_closing, MjText, "<mj-text />");
    crate::should_parse!(normal, MjText, "<mj-text>Hello World!</mj-text>");
}
