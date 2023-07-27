use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::MjAttributesClass;
use crate::prelude::hash::Map;
use crate::prelude::parser::{Error, Parsable, Parser, ParserOptions};

#[derive(Debug)]
struct MjAttributesClassParser {
    name: String,
    attributes: Map<String, String>,
}

impl MjAttributesClassParser {
    fn new(_opts: Rc<ParserOptions>) -> Self {
        Self {
            name: String::default(),
            attributes: Map::new(),
        }
    }
}

impl Parser for MjAttributesClassParser {
    type Output = MjAttributesClass;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MjAttributesClass {
            name: self.name,
            attributes: self.attributes,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "name" {
            self.name = value.to_string();
        } else {
            self.attributes.insert(name.to_string(), value.to_string());
        }
        Ok(())
    }
}

impl Parsable for MjAttributesClass {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjAttributesClassParser::new(opts).parse(tokenizer)?.build()
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
