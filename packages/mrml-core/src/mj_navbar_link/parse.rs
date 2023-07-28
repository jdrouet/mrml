use xmlparser::StrSpan;

use super::MjNavbarLink;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjNavbarLink> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjNavbarLink, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;

        if ending.empty {
            return Ok(MjNavbarLink {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjNavbarLink {
            attributes,
            children,
        })
    }
}
