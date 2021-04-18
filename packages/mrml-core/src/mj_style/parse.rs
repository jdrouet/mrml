use super::MJStyle;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJStyleParser(MJStyle);

impl Parser for MJStyleParser {
    type Output = MJStyle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "inline" {
            self.0.attributes.inline = Some(value.to_string());
            Ok(())
        } else {
            Err(Error::UnexpectedAttribute(name.start()))
        }
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.0.children = value.to_string();
        Ok(())
    }
}

impl Parsable for MJStyle {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJStyleParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-style>.whatever {background-color: red};</mj-style></mj-head></mjml>"#,
        );
        println!("result: {:?}", res);
        assert!(res.is_ok());
    }
}
