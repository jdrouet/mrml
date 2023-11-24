use xmlparser::StrSpan;

use super::MjAttributesElement;
use crate::prelude::hash::Map;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjAttributesElement> for MrmlCursor<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjAttributesElement, Error> {
        let attributes: Map<String, String> = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            self.assert_element_close()?;
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
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn parse_complete() {
        let raw = r#"
<mj-attributes>
    <mj-class name="whatever" color="red" />
</mj-attributes>
        "#;
        let _: MjAttributes = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
