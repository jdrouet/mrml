use xmlparser::StrSpan;

use super::MjAttributesAll;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

impl ParseElement<MjAttributesAll> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAttributesAll, Error> {
        let attributes = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            cursor.assert_element_close()?;
        }

        Ok(MjAttributesAll { attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes_all::MjAttributesAll;

    crate::should_sync_parse!(parse_complete, MjAttributesAll, r#"<mj-all color="red" />"#);
}
