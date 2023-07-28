use xmlparser::StrSpan;

use super::MjAccordionText;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjAccordionText> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionText, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.next_element_end()?.ok_or(Error::EndOfStream)?;
        let children = if !ending.empty {
            self.parse_children()?
        } else {
            Default::default()
        };

        Ok(MjAccordionText {
            attributes,
            children,
        })
    }
}
