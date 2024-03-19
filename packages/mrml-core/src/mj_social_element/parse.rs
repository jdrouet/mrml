use xmlparser::StrSpan;

use super::MjSocialElement;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjSocialElement> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjSocialElement, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjSocialElement {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjSocialElement> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _: StrSpan<'a>,
    ) -> Result<MjSocialElement, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjSocialElement {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_social_element::MjSocialElement;

    crate::should_sync_parse!(
        parse_with_empty_children,
        MjSocialElement,
        r#"<mj-social-element name="facebook" />"#
    );

    crate::should_sync_parse!(
        parse_ending_tag,
        MjSocialElement,
        r#"<mj-social-element name="facebook">
    Share <b>test</b> hi
</mj-social-element>"#
    );
}
