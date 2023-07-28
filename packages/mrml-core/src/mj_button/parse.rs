use xmlparser::StrSpan;

use super::MjButton;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjButton> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjButton, Error> {
        println!("mj-button open");
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjButton {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;

        self.assert_element_close()?;
        println!("mj-button close");

        Ok(MjButton {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{mj_button::MjButton, prelude::parser::MrmlParser};

    #[test]
    fn success() {
        let raw = r#"<mj-button>
    <!-- Just a comment -->
    <b>foo</b>
    bar
</mj-button>"#;
        let _: MjButton = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
