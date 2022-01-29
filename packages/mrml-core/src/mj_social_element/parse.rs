use super::MJSocialElement;
use crate::mj_raw::MJRawChild;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJSocialElementParser(MJSocialElement);

impl Parser for MJSocialElementParser {
    type Output = MJSocialElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_comment!();
    parse_child!(MJRawChild);
    parse_text!();
}

impl Parsable for MJSocialElement {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJSocialElementParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;

    #[test]
    fn parse_ending_tag() {
        let template = r#"
        <mjml>
          <mj-body>
            <mj-social>
              <mj-social-element name="facebook">
                Share <b>test</b> hi
              </mj-social-element>
            </mj-social>
          </mj-body>
        </mjml>
        "#;
        MJML::parse(template.to_string()).unwrap();
    }
}
