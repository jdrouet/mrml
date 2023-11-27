use xmlparser::StrSpan;

use super::MjSocialElement;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjSocialElement> for MrmlParser {
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

#[cfg(test)]
mod tests {
    use crate::mj_social_element::MjSocialElement;

    crate::should_parse!(
        parse_with_empty_children,
        MjSocialElement,
        r#"<mj-social-element name="facebook" />"#
    );

    crate::should_parse!(
        parse_ending_tag,
        MjSocialElement,
        r#"<mj-social-element name="facebook">
    Share <b>test</b> hi
</mj-social-element>"#
    );
}
