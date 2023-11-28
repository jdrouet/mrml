use xmlparser::StrSpan;

use super::MjAttributesElement;
use crate::prelude::hash::Map;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

impl ParseElement<MjAttributesElement> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesElement, Error> {
        let attributes: Map<String, String> = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            cursor.assert_element_close()?;
        }

        Ok(MjAttributesElement {
            name: tag.to_string(),
            attributes,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes::MjAttributes;

    crate::should_sync_parse!(
        parse_complete,
        MjAttributes,
        r#"<mj-attributes><mj-class name="whatever" color="red" /></mj-attributes>"#
    );
}
