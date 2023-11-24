use xmlparser::StrSpan;

use super::MjColumn;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjColumn> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjColumn, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjColumn {
            attributes,
            children,
        })
    }
}
