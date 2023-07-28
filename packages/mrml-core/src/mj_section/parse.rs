use xmlparser::StrSpan;

use super::MjSection;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjSection> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjSection, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjSection::default());
        }

        let children = self.parse_children()?;

        self.assert_element_close()?;

        Ok(MjSection {
            attributes,
            children,
        })
    }
}
