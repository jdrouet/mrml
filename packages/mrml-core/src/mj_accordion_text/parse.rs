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
