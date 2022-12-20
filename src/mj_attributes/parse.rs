use super::{MJAttributes, MJAttributesChild};
use crate::mj_attributes_all::MJAttributesAll;
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::MJAttributesClass;
use crate::mj_attributes_class::NAME as MJ_CLASS;
use crate::mj_attributes_element::MJAttributesElement;
use crate::parse_child;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MJAttributesChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ALL => Ok(MJAttributesAll::parse(tag, tokenizer)?.into()),
            MJ_CLASS => Ok(MJAttributesClass::parse(tag, tokenizer)?.into()),
            _ => Ok(MJAttributesElement::parse(tag, tokenizer)?.into()),
        }
    }
}

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
        let elt = MJML::parse(template).unwrap();
        assert!(elt.head().is_some());
        assert!(elt.body().is_none());
        let head = elt.head().unwrap();
        assert_eq!(head.children().len(), 1);
    }
}
