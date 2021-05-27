use super::MJAttributesClass;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJAttributesClassParser(MJAttributesClass);

impl Parser for MJAttributesClassParser {
    type Output = MJAttributesClass;

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

impl Parsable for MJAttributesClass {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAttributesClassParser::default().parse(tokenizer)?.build()
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
        let elt = MJML::parse(template.to_string()).unwrap();
        assert!(elt.head().is_some());
        assert!(elt.body().is_none());
        let head = elt.head().unwrap();
        assert_eq!(head.children().len(), 1);
    }
}
