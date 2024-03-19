use xmlparser::StrSpan;

use super::MjAccordionTitle;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseChildren, ParseElement};
use crate::text::Text;

#[inline]
fn parse_children(cursor: &mut MrmlCursor<'_>) -> Result<Vec<Text>, Error> {
    let mut result = Vec::new();

    while let Some(item) = cursor.next_text()? {
        if !item.text.trim().is_empty() {
            result.push(Text::from(item.text.as_str()));
        }
    }

    Ok(result)
}

impl<'opts> ParseChildren<Vec<Text>> for MrmlParser<'opts> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<Text>, Error> {
        parse_children(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<Text>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<Text>, Error> {
        parse_children(cursor)
    }
}

impl<'opts> ParseElement<MjAccordionTitle> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordionTitle, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjAccordionTitle {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjAccordionTitle> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordionTitle, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjAccordionTitle {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionTitle;

    crate::should_sync_parse!(
        should_work_with_child_text,
        MjAccordionTitle,
        "<mj-accordion-title>Hello</mj-accordion-title>"
    );

    crate::should_sync_parse!(
        should_work_with_no_children,
        MjAccordionTitle,
        "<mj-accordion-title />"
    );

    crate::should_not_sync_parse!(
        should_error_with_no_closing,
        MjAccordionTitle,
        "<mj-accordion-title>",
        "EndOfStream"
    );
}
