use xmlparser::StrSpan;

use super::MjHero;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjHero> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjHero, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjHero {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;

        self.assert_element_close()?;

        Ok(MjHero {
            attributes,
            children,
        })
    }
}
