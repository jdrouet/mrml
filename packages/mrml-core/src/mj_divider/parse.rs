use xmlparser::StrSpan;

use super::MjDivider;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjDivider> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjDivider, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            self.assert_element_close()?;
        }

        Ok(MjDivider { attributes })
    }
}
