use xmlparser::StrSpan;

use super::MjTable;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjTable> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjTable, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjTable {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;

        self.assert_element_close()?;

        Ok(MjTable {
            attributes,
            children,
        })
    }
}
