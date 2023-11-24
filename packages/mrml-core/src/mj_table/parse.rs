use xmlparser::StrSpan;

use super::MjTable;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjTable> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjTable, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjTable {
            attributes,
            children,
        })
    }
}
