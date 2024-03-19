use xmlparser::StrSpan;

use super::MjAccordionText;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjAccordionText> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordionText, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjAccordionText {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjAccordionText> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordionText, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjAccordionText {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionText;

    crate::should_sync_parse!(
        should_work_with_child_text,
        MjAccordionText,
        "<mj-accordion-text>Hello</mj-accordion-text>"
    );

    crate::should_sync_parse!(
        should_work_with_no_children,
        MjAccordionText,
        "<mj-accordion-text />"
    );

    crate::should_not_sync_parse!(
        should_error_with_no_closing,
        MjAccordionText,
        "<mj-accordion-text>",
        "EndOfStream"
    );
}
