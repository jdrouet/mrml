use xmlparser::StrSpan;

use super::MjSocialElement;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjSocialElement> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjSocialElement, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjSocialElement {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_social_element::MjSocialElement;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn parse_with_empty_children() {
        let template = r#"<mj-social-element name="facebook" />"#;
        let _: MjSocialElement = MrmlCursor::new(template, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn parse_ending_tag() {
        let template = r#"
<mj-social-element name="facebook">
    Share <b>test</b> hi
</mj-social-element>
        "#;
        let _: MjSocialElement = MrmlCursor::new(template, Default::default())
            .parse_root()
            .unwrap();
    }
}
