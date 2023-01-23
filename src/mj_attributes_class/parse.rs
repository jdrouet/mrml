use super::MjAttributesClass;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MjAttributesClassParser(MjAttributesClass);

impl Parser for MjAttributesClassParser {
    type Output = MjAttributesClass;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "name" {
            self.0.name = value.to_string();
        } else {
            self.0
                .attributes
                .insert(name.to_string(), value.to_string());
        }
        Ok(())
    }
}

impl Parsable for MjAttributesClass {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MjAttributesClassParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;

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
        let elt = MJML::parse(template).unwrap();
        assert!(elt.head().is_some());
        assert!(elt.body().is_none());
        let head = elt.head().unwrap();
        assert_eq!(head.children().len(), 1);
    }
}
