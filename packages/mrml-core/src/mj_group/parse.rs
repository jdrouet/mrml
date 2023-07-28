use xmlparser::StrSpan;

use super::MjGroup;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjGroup> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjGroup, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjGroup {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;

        self.assert_element_close()?;

        Ok(MjGroup {
            attributes,
            children,
        })
    }
}
