use xmlparser::StrSpan;

use super::MjSpacer;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

impl ParseElement<MjSpacer> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjSpacer, Error> {
        let attributes: crate::prelude::hash::Map<String, String> =
            self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            cursor.assert_element_close()?;
        }

        Ok(MjSpacer { attributes })
    }
}
