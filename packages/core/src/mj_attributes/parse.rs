use super::{MJAttributes, MJAttributesChild};
use crate::parse_child;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJAttributesParser(MJAttributes);

impl Parser for MJAttributesParser {
    type Output = MJAttributes;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_child!(MJAttributesChild);
}

impl Parsable for MJAttributes {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAttributesParser::default().parse(tokenizer)?.build()
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
                    <mj-all color="red" />
                    <mj-class name="head" color="green" />
                    <span color="blue" />
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
