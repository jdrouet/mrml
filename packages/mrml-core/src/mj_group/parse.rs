use xmlparser::StrSpan;

use super::MjGroup;
use crate::prelude::parser::{ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjGroup> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjGroup, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjGroup {
            attributes,
            children,
        })
    }
}
