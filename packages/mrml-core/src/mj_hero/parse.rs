use xmlparser::StrSpan;

use super::MjHero;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjHero> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjHero, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjHero {
            attributes,
            children,
        })
    }
}
