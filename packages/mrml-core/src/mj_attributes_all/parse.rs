use xmlparser::StrSpan;

use super::MjAttributesAll;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjAttributesAll> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAttributesAll, Error> {
        let attributes = self.parse_attributes()?;

        Ok(MjAttributesAll { attributes })
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
