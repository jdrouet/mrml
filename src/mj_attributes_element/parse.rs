use super::MjAttributesElement;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
struct MjAttributesElementParser(MjAttributesElement);

impl MjAttributesElementParser {
    pub fn new(name: String) -> Self {
        Self(MjAttributesElement::new(name))
    }
}

impl Parser for MjAttributesElementParser {
    type Output = MjAttributesElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "name" {
            self.0.name = name.to_string();
        } else {
            self.0
                .attributes
                .insert(name.to_string(), value.to_string());
        }
        Ok(())
    }
}

impl Parsable for MjAttributesElement {
    fn parse(tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MjAttributesElementParser::new(tag.to_string())
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
