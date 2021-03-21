use super::{MJBody, MJBodyChild};
use crate::prelude::parse::{Error, Parser};
use crate::{parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJBodyParser(MJBody);

impl Parser for MJBodyParser {
    type Output = MJBody;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_comment!();
    parse_text!();
    parse_child!(MJBodyChild);
}

impl MJBody {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJBodyParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;

    #[test]
    fn parse_complete() {
        let template = r#"
        <mjml>
            <mj-body>
                <!-- Some comment -->
                <mj-button>Hello World</mj-button>
            </mj-body>
        </mjml>
        "#;
        let elt = MJML::parse(template.to_string()).unwrap();
        assert!(elt.head().is_none());
        assert!(elt.body().is_some());
        let body = elt.body().unwrap();
        assert_eq!(body.children.len(), 2);
    }
}
