use xmlparser::StrSpan;

use super::MjAttributesClass;
use crate::prelude::hash::Map;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjAttributesClass> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAttributesClass, Error> {
        let mut attributes: Map<String, String> = self.parse_attributes()?;
        let name: String = attributes
            .remove("name")
            .ok_or_else(|| Error::MissingAttribute("name"))?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            self.assert_element_close()?;
        }

        Ok(MjAttributesClass { name, attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::{mj_attributes_class::MjAttributesClass, prelude::parser::MrmlParser};

    #[test]
    fn parse_complete() {
        let raw = r#"<mj-class name="whatever" color="red" />"#;
        let _: MjAttributesClass = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "MissingAttribute(\"name\")")]
    fn should_have_name() {
        let raw = r#"<mj-class color="red" />"#;
        let _: MjAttributesClass = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedToken(33, 42)")]
    fn should_close() {
        let raw = r#"<mj-class name="div" color="red"><whatever>"#;
        let _: MjAttributesClass = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
