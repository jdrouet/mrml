use xmlparser::StrSpan;

use super::MjAttributesClass;
use crate::prelude::hash::Map;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjAttributesClass> for MrmlCursor<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjAttributesClass, Error> {
        let mut attributes: Map<String, String> = self.parse_attributes()?;
        let name: String = attributes
            .remove("name")
            .ok_or_else(|| Error::MissingAttribute("name", tag.into()))?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            self.assert_element_close()?;
        }

        Ok(MjAttributesClass { name, attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes_class::MjAttributesClass;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn parse_complete() {
        let raw = r#"<mj-class name="whatever" color="red" />"#;
        let _: MjAttributesClass = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "MissingAttribute(\"name\", Span { start: 1, end: 9 })")]
    fn should_have_name() {
        let raw = r#"<mj-class color="red" />"#;
        let _: MjAttributesClass = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedToken(Span { start: 33, end: 42 })")]
    fn should_close() {
        let raw = r#"<mj-class name="div" color="red"><whatever>"#;
        let _: MjAttributesClass = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
