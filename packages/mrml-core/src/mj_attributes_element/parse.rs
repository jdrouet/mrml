use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::MjAttributesElement;
use crate::prelude::hash::Map;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};
use crate::prelude::parser::{Parsable, Parser, ParserOptions};

impl<'a> ElementParser<'a, MjAttributesElement> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAttributesElement, Error> {
        let mut attributes: Map<String, String> = self.parse_attributes()?;
        let name: String = attributes
            .remove("name")
            .ok_or_else(|| Error::MissingAttribute("name"))?;

        Ok(MjAttributesElement { name, attributes })
    }
}

#[derive(Debug)]
struct MjAttributesElementParser {
    name: String,
    attributes: Map<String, String>,
}

impl MjAttributesElementParser {
    pub fn new(name: String, _opts: Rc<ParserOptions>) -> Self {
        Self {
            name,
            attributes: Map::new(),
        }
    }
}

impl Parser for MjAttributesElementParser {
    type Output = MjAttributesElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MjAttributesElement {
            name: self.name,
            attributes: self.attributes,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "name" {
            self.name = name.to_string();
        } else {
            self.attributes.insert(name.to_string(), value.to_string());
        }
        Ok(())
    }
}

impl Parsable for MjAttributesElement {
    fn parse(
        tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjAttributesElementParser::new(tag.to_string(), opts)
            .parse(tokenizer)?
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::Mjml;

    #[test]
    fn parse_complete() {
        let template = r#"
        <mjml>
            <mj-head>
                <mj-attributes>
                    <mj-class name="whatever" color="red" />
                </mj-attributes>
            </mj-head>
        </mjml>
        "#;
        let elt = Mjml::parse(template).unwrap();
        assert!(elt.head().is_some());
        assert!(elt.body().is_none());
        let head = elt.head().unwrap();
        assert_eq!(head.children().len(), 1);
    }
}
