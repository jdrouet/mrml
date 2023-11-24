use xmlparser::StrSpan;

use super::MjSection;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjSection> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjSection, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjSection {
            attributes,
            children,
        })
    }
}
