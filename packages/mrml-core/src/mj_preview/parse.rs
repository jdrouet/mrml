use xmlparser::StrSpan;

use super::MjPreview;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjPreview> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjPreview, Error> {
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
