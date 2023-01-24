use super::MjAttributesChild;
use crate::mj_attributes_all::MjAttributesAll;
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::MjAttributesClass;
use crate::mj_attributes_class::NAME as MJ_CLASS;
use crate::mj_attributes_element::MjAttributesElement;
use crate::prelude::parse::{Error, Parsable};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjAttributesChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ALL => Ok(MjAttributesAll::parse(tag, tokenizer)?.into()),
            MJ_CLASS => Ok(MjAttributesClass::parse(tag, tokenizer)?.into()),
            _ => Ok(MjAttributesElement::parse(tag, tokenizer)?.into()),
        }
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
                    <mj-all color="red" />
                    <mj-class name="head" color="green" />
                    <span color="blue" />
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
