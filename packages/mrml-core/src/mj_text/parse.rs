use xmlparser::StrSpan;

use super::MjText;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjText> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjText, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjText {
            attributes,
            children,
        })
    }
}
