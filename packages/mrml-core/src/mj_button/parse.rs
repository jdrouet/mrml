use xmlparser::StrSpan;

use super::MjButton;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjButton> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjButton, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjButton {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl crate::prelude::parser::AsyncParseElement<MjButton> for MrmlParser {
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

    crate::should_async_parse!(
        async_success,
        MjButton,
        r#"<mj-button>
    <!-- Just a comment -->
    <b>foo</b>
    bar
</mj-button>"#
    );
}
