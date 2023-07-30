use xmlparser::StrSpan;

use super::MjAccordionText;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjAccordionText> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionText, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjAccordionText {
                attributes,
                children: Vec::new(),
            });
        }
        let children = self.parse_children()?;
        let _ = self.assert_element_close()?;

        Ok(MjAccordionText {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionText;
    use crate::prelude::parser::MrmlParser;

    #[test]
    fn should_work_with_child_text() {
        let raw = "<mj-accordion-text>Hello</mj-accordion-text>";
        let _: MjAccordionText = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn should_work_with_no_children() {
        let raw = "<mj-accordion-text />";
        let _: MjAccordionText = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "EndOfStream")]
    fn should_error_with_no_closing() {
        let raw = "<mj-accordion-text>";
        let _: MjAccordionText = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
