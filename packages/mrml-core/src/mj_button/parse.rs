use xmlparser::StrSpan;

use super::MjButton;
use crate::prelude::parser::{AsyncParseElement, Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjButton> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjButton, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjButton {
            attributes,
            children,
        })
    }
}

#[async_trait::async_trait]
impl AsyncParseElement<MjButton> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjButton, Error> {
        let (attributes, children) = self.async_parse_attributes_and_children(cursor).await?;

        Ok(MjButton {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;

    crate::should_parse!(
        success,
        MjButton,
        r#"<mj-button>
    <!-- Just a comment -->
    <b>foo</b>
    bar
</mj-button>"#
    );
}
