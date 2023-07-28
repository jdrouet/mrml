use xmlparser::StrSpan;

use super::MjSocialElement;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjSocialElement> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjSocialElement, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;

        if ending.empty {
            return Ok(MjSocialElement {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjSocialElement {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{mj_social_element::MjSocialElement, prelude::parser::MrmlParser};

    #[test]
    fn parse_ending_tag() {
        let template = r#"
<mj-social-element name="facebook">
    Share <b>test</b> hi
</mj-social-element>
        "#;
        let _: MjSocialElement = MrmlParser::new(template, Default::default())
            .parse_root()
            .unwrap();
    }
}
