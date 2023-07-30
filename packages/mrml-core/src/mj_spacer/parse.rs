use xmlparser::StrSpan;

use super::MjSpacer;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjSpacer> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjSpacer, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            self.assert_element_close()?;
        }

        Ok(MjSpacer { attributes })
    }
}
