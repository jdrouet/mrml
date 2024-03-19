use xmlparser::StrSpan;

use super::MjWrapper;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjWrapper> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjWrapper, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjWrapper {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjWrapper> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjWrapper, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjWrapper {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_wrapper::MjWrapper;

    crate::should_sync_parse!(
        parse_br_element,
        MjWrapper,
        "<mj-wrapper><h1>hello</h1><br><h2>world</h2></mj-wrapper>"
    );
}
