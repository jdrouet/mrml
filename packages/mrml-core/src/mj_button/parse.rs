use xmlparser::StrSpan;

use super::MjButton;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjButton> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjButton, Error> {
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

        Ok(MjButton {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        crate::mjml::Mjml::parse(
            r#"<mjml>
    <mj-body>
        <mj-button>
            <!-- Just a comment -->
            <b>foo</b>
            bar
        </mj-button>
    </mj-body>
</mjml>"#,
        )
        .unwrap();
    }
}
